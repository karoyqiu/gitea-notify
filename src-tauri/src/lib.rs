mod gitea;

use std::time::Duration;
use std::{collections::HashMap, sync::LazyLock};

use reqwest;
use serde::Deserialize;
use tauri::{
  AppHandle, Manager, Result, WebviewWindowBuilder,
  menu::{Menu, MenuItem, PredefinedMenuItem},
  tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_store::{self as plugin_store, StoreExt};
use tokio::{sync::Mutex, time};

use crate::gitea::Gitea;

/// Gitea API
static GITEA: LazyLock<Mutex<Gitea>> =
  LazyLock::new(|| Mutex::new(Gitea::new("".to_string(), "".to_string())));

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

async fn check_repo_workflow_runs(app: &AppHandle, gitea: &Gitea) -> reqwest::Result<()> {
  let repos = gitea.user_current_list_repos(None, Some(100)).await?;

  // TODO: 仓库白名单

  let mut known_runs = KNOWN_RUNS.lock().await;
  let mut lines: Vec<String> = Vec::new();

  for repo in repos {
    // 只取最近一次执行
    let runs = gitea.get_workflow_runs(&repo, Some(1), Some(1)).await?;

    if let Some(runs) = runs.workflow_runs {
      for run in runs {
        let key = format!("{}#{}", repo.full_name, run.run_number);
        let line = format!("{}:\n{} - {}", key, run.status, run.conclusion.unwrap_or("running".into()));

        if let Some(known_status) = known_runs.insert(key, run.status.clone()) {
          // 已经见过了，比较当前状态与之前的状态
          if known_status != run.status {
            lines.push(line);
          }
        } else {
          lines.push(line);
        }
      }
    }
  }

  if !lines.is_empty() {
    // 统一通知
    let title = String::from("工作流运行状态变更");
    let body = lines.join("\n\n");
    let _ = app.notification().builder().title(title).body(body).show();
  }

  Ok(())
}

/// 周期任务主逻辑：从 store 读取配置并轮询所有仓库
async fn periodic_task_for_app(app: AppHandle) {
  // read settings (try plugin then fallback file)
  if let Some(settings) = read_settings(&app) {
    let mut gitea = GITEA.lock().await;
    gitea.reset(settings.url, settings.token);

    if let Err(e) = check_repo_workflow_runs(&app, &gitea).await {
      eprintln!("Error: {}", e);
    }
  } else {
    eprintln!("settings not found; skipping this tick");
  }
}

/// 启动周期任务。把 owned AppHandle 移动到后台任务，以便在任务中使用 plugin API。
fn start_periodic_task_with_app(app_handle: AppHandle) {
  tauri::async_runtime::spawn(async move {
    let mut interval = time::interval(Duration::from_millis(PERIODIC_INTERVAL_MILLIS));

    loop {
      interval.tick().await;

      let fut = periodic_task_for_app(app_handle.clone());
      match time::timeout(Duration::from_millis(TASK_TIMEOUT_MILLIS), fut).await {
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
