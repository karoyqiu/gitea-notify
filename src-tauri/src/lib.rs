use tauri::{
  AppHandle, Manager, Result, WebviewWindowBuilder,
  menu::{Menu, MenuItem, PredefinedMenuItem},
  tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
};

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
