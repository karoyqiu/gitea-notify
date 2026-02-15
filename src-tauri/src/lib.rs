use std::sync::Mutex;
use std::time::Duration;
use std::{collections::HashMap, sync::LazyLock};

use reqwest;
use serde::Deserialize;
use serde_json::Value;
use tauri::{
  AppHandle, Manager, Result, WebviewWindowBuilder,
  menu::{Menu, MenuItem, PredefinedMenuItem},
  tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_store::{self as plugin_store, StoreExt};
use tokio;

/// 进程内已见 run 状态缓存（不持久化）
/// Key: "{owner/repo}#{run_id}" -> last seen state string
static KNOWN_RUNS: LazyLock<Mutex<HashMap<String, String>>> =
  LazyLock::new(|| Mutex::new(HashMap::new()));

/// 轮询间隔与单次超时（秒）
const PERIODIC_INTERVAL_MILLIS: u64 = 1500;
const TASK_TIMEOUT_MILLIS: u64 = 1000;

#[derive(Debug, Deserialize)]
struct Settings {
  url: String,
  token: String,
  /// 可选仓库白名单 "owner/repo"
  repos: Option<Vec<String>>,
}

/// 统一读取配置：优先尝试 plugin API（同步），若失败再回退到文件读取（异步）。
fn read_settings(app: &AppHandle) -> Option<Settings> {
  if let Ok(store) = app.store("store.json") {
    let settings = store.get("settings");
    store.close_resource();

    if let Some(settings) = settings {
      if let Ok(settings) = serde_json::from_value::<Settings>(settings) {
        return Some(settings);
      }
    }
  }

  None
}

/// 通过插件发送通知（优先）或回退到系统通知（notify-rust via plugin or native）
fn notify_via_plugin_or_fallback(app: &AppHandle, title: &str, body: &str) {
  // Using plugin notification API exposed on AppHandle
  // The plugin provides `app.notification()` via trait `NotificationExt` (per sample).
  // We call builder and show; if it errors, fallback to a best-effort local notify using the plugin itself if available.
  let app = app.clone();
  let title = title.to_string();
  let body = body.to_string();

  // We call `app.notification()` (provided by the plugin via the trait).
  // Use `.builder()` -> `.title()`/`.body()` -> `.show()`.
  // This call may return a Result; ignore/show errors gracefully.
  if let Err(e) = app
    .notification()
    .builder()
    .title(&title)
    .body(&body)
    .show()
  {
    eprintln!("plugin notification show error: {:?}", e);
  }
}

/// 列出用户仓库（owner/repo 列表），若失败返回空 vec
async fn list_repos_from_gitea(
  base_url: &str,
  token: &str,
  client: &reqwest::Client,
) -> Vec<String> {
  let url = format!("{}/api/v1/user/repos", base_url.trim_end_matches('/'));
  let resp = match client.get(&url).bearer_auth(token).send().await {
    Ok(r) => r,
    Err(e) => {
      eprintln!("failed to request repos: {:?}", e);
      return Vec::new();
    }
  };

  if !resp.status().is_success() {
    eprintln!("repos request returned status: {}", resp.status());
    return Vec::new();
  }

  let mut out = Vec::new();
  match resp.json::<Value>().await {
    Ok(v) => {
      if let Some(arr) = v.as_array() {
        for repo in arr {
          if let Some(fn_) = repo.get("full_name").and_then(|s| s.as_str()) {
            out.push(fn_.to_string());
          } else {
            let owner = repo
              .get("owner")
              .and_then(|o| o.get("login"))
              .and_then(|s| s.as_str())
              .unwrap_or("");
            let name = repo.get("name").and_then(|s| s.as_str()).unwrap_or("");
            if !owner.is_empty() && !name.is_empty() {
              out.push(format!("{}/{}", owner, name));
            }
          }
        }
      }
    }
    Err(e) => {
      eprintln!("failed to parse repos json: {:?}", e);
    }
  }

  out
}

/// 拉取单个仓库的 workflow runs 并检测新建/完成事件，发送通知
async fn poll_repo_runs_with_app(
  base_url: &str,
  token: &str,
  client: &reqwest::Client,
  app: &AppHandle,
  repo: &str,
) {
  let parts: Vec<&str> = repo.splitn(2, '/').collect();
  if parts.len() != 2 {
    return;
  }
  let owner = parts[0];
  let name = parts[1];

  let url = format!(
    "{}/api/v1/repos/{}/{}/actions/runs",
    base_url.trim_end_matches('/'),
    owner,
    name
  );
  let resp = match client.get(&url).bearer_auth(token).send().await {
    Ok(r) => r,
    Err(e) => {
      eprintln!("failed to fetch runs for {}: {:?}", repo, e);
      return;
    }
  };

  if !resp.status().is_success() {
    eprintln!("runs request for {} returned {}", repo, resp.status());
    return;
  }

  let json = match resp.json::<Value>().await {
    Ok(j) => j,
    Err(e) => {
      eprintln!("failed to parse runs json for {}: {:?}", repo, e);
      return;
    }
  };

  // 支持多种字段形式：workflow_runs、runs、或根数组
  let runs = if let Some(arr) = json.get("workflow_runs").and_then(|v| v.as_array()) {
    arr.clone()
  } else if let Some(arr) = json.get("runs").and_then(|v| v.as_array()) {
    arr.clone()
  } else if json.is_array() {
    json.as_array().unwrap().clone()
  } else {
    Vec::new()
  };

  let mut map = KNOWN_RUNS.lock().unwrap();

  for run in runs {
    let id_opt = run
      .get("id")
      .and_then(|v| v.as_i64())
      .or_else(|| run.get("run_id").and_then(|v| v.as_i64()))
      .or_else(|| run.get("number").and_then(|v| v.as_i64()));

    if id_opt.is_none() {
      continue;
    }
    let id = id_opt.unwrap().to_string();

    let status = run
      .get("status")
      .and_then(|v| v.as_str())
      .unwrap_or("")
      .to_string();
    let conclusion = run
      .get("conclusion")
      .and_then(|v| v.as_str())
      .unwrap_or("")
      .to_string();
    let _url_field = run
      .get("html_url")
      .and_then(|v| v.as_str())
      .or_else(|| run.get("url").and_then(|v| v.as_str()))
      .map(|s| s.to_string());

    let key = format!("{}#{}", repo, id);

    match map.get(&key) {
      None => {
        // New run
        map.insert(key.clone(), status.clone());
        let title = format!("Workflow started - {}", repo);
        let body = format!(
          "Run {} started (status={})",
          id,
          if status.is_empty() {
            "unknown"
          } else {
            &status
          }
        );

        // send plugin notification (preferred) or fallback
        notify_via_plugin_or_fallback(app, &title, &body);
      }
      Some(prev) => {
        // 检查从非终态到终态
        let was_terminal = {
          let l = prev.to_lowercase();
          l == "completed" || l == "done" || l == "success" || l == "failure" || l == "cancelled"
        };
        let is_terminal = {
          let s = status.to_lowercase();
          let c = conclusion.to_lowercase();
          s == "completed"
            || s == "done"
            || c == "success"
            || c == "failure"
            || c == "cancelled"
            || s == "failure"
            || s == "success"
        };

        if !was_terminal && is_terminal {
          map.insert(key.clone(), status.clone());
          let title = format!("Workflow finished - {}", repo);
          let body = if !conclusion.is_empty() {
            format!("Run {} finished: {}", id, conclusion)
          } else if !status.is_empty() {
            format!("Run {} finished: {}", id, status)
          } else {
            format!("Run {} finished", id)
          };

          // send notification
          notify_via_plugin_or_fallback(app, &title, &body);
        } else {
          // 更新状态
          map.insert(key.clone(), status.clone());
        }
      }
    }
  }
}

/// 周期任务主逻辑：从 store 读取配置并轮询所有仓库
async fn periodic_task_for_app(app: AppHandle) {
  // read settings (try plugin then fallback file)
  if let Some(settings) = read_settings(&app) {
    let client = reqwest::Client::new();

    // decide repo list
    let repos: Vec<String> = if let Some(list) = settings.repos.clone() {
      list
    } else {
      list_repos_from_gitea(&settings.url, &settings.token, &client).await
    };

    for repo in repos {
      poll_repo_runs_with_app(&settings.url, &settings.token, &client, &app, &repo).await;
    }
  } else {
    eprintln!("settings not found; skipping this tick");
  }
}

/// 启动周期任务。把 owned AppHandle 移动到后台任务，以便在任务中使用 plugin API。
fn start_periodic_task_with_app(app_handle: AppHandle) {
  tauri::async_runtime::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_millis(PERIODIC_INTERVAL_MILLIS));

    loop {
      interval.tick().await;

      let fut = periodic_task_for_app(app_handle.clone());
      match tokio::time::timeout(Duration::from_millis(TASK_TIMEOUT_MILLIS), fut).await {
        Ok(_) => {}
        Err(_) => eprintln!("periodic task timed out after {}s", TASK_TIMEOUT_MILLIS),
      }
    }
  });
}

/// 显示主窗口。如果没有主窗口，则根据配置创建一个。
fn show_main_window(app: &AppHandle) -> Result<()> {
  if let Some(window) = app.get_webview_window("main") {
    window.show()?;
    window.set_focus()?;
    window.request_user_attention(Some(tauri::UserAttentionType::Informational))?;
  } else {
    WebviewWindowBuilder::from_config(app, &app.config().app.windows.get(0).unwrap())?.build()?;
  }

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(plugin_store::Builder::default().build())
    .plugin(tauri_plugin_notification::init())
    .setup(|app| {
      let settings = MenuItem::with_id(app, "settings", "&Settings", true, None::<&str>)?;
      let sep = PredefinedMenuItem::separator(app)?;
      let quit = PredefinedMenuItem::quit(app, None)?;
      let menu = Menu::with_items(app, &[&settings, &sep, &quit])?;

      let _tray = TrayIconBuilder::new()
        .title("Gitea notifications")
        .tooltip("Gitea notifications")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
          "settings" => {
            show_main_window(app).expect("Failed to show main window");
          }
          _ => {
            println!("menu item {:?} not handled", event.id);
          }
        })
        .on_tray_icon_event(|tray, event| match event {
          TrayIconEvent::DoubleClick {
            button: MouseButton::Left,
            ..
          } => {
            let app = tray.app_handle();
            show_main_window(app).expect("Failed to show main window");
          }
          _ => {}
        })
        .show_menu_on_left_click(false)
        .build(app)?;

      // 启动周期任务（传入 owned AppHandle，以便在后台使用插件 API）
      start_periodic_task_with_app(app.handle().to_owned());

      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    });
}
