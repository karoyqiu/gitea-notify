use tauri::{
  AppHandle, Manager, Result, WebviewWindowBuilder,
  menu::{Menu, MenuItem, PredefinedMenuItem},
  tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
};

/// 周期性任务的间隔（秒）
const PERIODIC_INTERVAL_MILLIS: u64 = 1500;
/// 单次任务的超时（秒）
const TASK_TIMEOUT_MILLIS: u64 = 1000;

/// 要重复执行的异步任务实现。把实际业务逻辑放在这里。
async fn periodic_task_async(_app_handle: AppHandle) {
  // 示例：打印时间并可选地与前端交互/发事件
  let now = std::time::SystemTime::now();
  match now.duration_since(std::time::UNIX_EPOCH) {
    Ok(dur) => {
      println!("[periodic_task] tick at unix secs: {}", dur.as_secs());
    }
    Err(_) => {
      println!("[periodic_task] tick at unknown time");
    }
  }

  // 如果需要与前端交互或发送事件，可使用 `_app_handle.emit_all(...)` 等 API（示例为注释形式）:
  // let _ = _app_handle.emit_all("periodic-tick", serde_json::json!({ "time": dur.as_secs() }));
}

/// 使用异步 interval + timeout 在后台循环执行任务。
/// 将在 `setup` 内部以 `AppHandle` 启动，从而可以在任务中与 Tauri 交互。
/// 接受一个 owned `AppHandle`，直接移动到异步任务内部。
fn start_periodic_task_with_interval(app_handle: AppHandle) {
  // 已经是 owned 的 AppHandle，直接移动到 async 任务中（需要多次使用时可 clone）
  let owned_handle = app_handle;

  // 通过 tauri 的 async runtime 启动任务
  tauri::async_runtime::spawn(async move {
    // 注意：我们这里使用 tokio 的 time 工具（通常可在 tauri 项目中使用）。
    // 如果你的项目没有直接依赖 tokio，你可能需要在 Cargo.toml 中显式添加 tokio 依赖或使用 tauri 提供的运行时封装方法。
    let mut ticker = tokio::time::interval(std::time::Duration::from_millis(PERIODIC_INTERVAL_MILLIS));
    loop {
      // 等待下一个 tick（首次 tick 立即触发后会等待间隔）
      ticker.tick().await;

      // 使用 timeout 来避免单次任务无限阻塞
      let task_fut = periodic_task_async(owned_handle.clone());
      match tokio::time::timeout(std::time::Duration::from_millis(TASK_TIMEOUT_MILLIS), task_fut).await {
        Ok(_) => {
          // 任务正常完成（或至少在超时内返回）
        }
        Err(_) => {
          // 超时
          eprintln!("[periodic_task] task timed out after {}ms", TASK_TIMEOUT_MILLIS);
        }
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
    .plugin(tauri_plugin_store::Builder::new().build())
    .setup(|app| {
      let settings = MenuItem::with_id(app, "settings", "&Settings", true, None::<&str>)?;
      let sep = PredefinedMenuItem::separator(app)?;
      let quit = PredefinedMenuItem::quit(app, None)?;
      let menu = Menu::with_items(app, &[&settings,&sep,&quit])?;

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
          TrayIconEvent::DoubleClick { button: MouseButton::Left, .. } => {
            let app = tray.app_handle();
            show_main_window(app).expect("Failed to show main window");
          }
          _ => {}
        })
        .show_menu_on_left_click(false)
        .build(app)?;

      // 在 setup 内部启动异步周期任务，以便任务可以使用 `AppHandle`
      start_periodic_task_with_interval(app.handle().to_owned());

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
