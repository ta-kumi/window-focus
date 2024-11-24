use tauri::{
    menu::{Menu, MenuItem, CheckMenuItem},
    tray::TrayIconBuilder,
};
use std::sync::{Arc, Mutex};

mod platform;
use platform::{
    MainModule,
    windows::Windows,
};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let wf_module = Arc::new(Mutex::new(Windows::new()));
            wf_module.lock().unwrap().initialize();

            let wf_arc = Arc::clone(&wf_module);
            let enable = CheckMenuItem::with_id(app, "enable", "Enable", true, true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&enable, &quit])?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "enable" => {
                            if enable.is_checked().unwrap() {
                                wf_arc.lock().unwrap().focus_on();
                                let _ = enable.set_checked(true);
                            }
                            else {
                                wf_arc.lock().unwrap().focus_off();
                                let _ = enable.set_checked(false);
                            }
                        }
                        "quit" => {
                            wf_arc.lock().unwrap().finialize();
                            app.exit(0);
                        }
                        _ => {
                            println!("menu item {:?} not handled", event.id());
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
