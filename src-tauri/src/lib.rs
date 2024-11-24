use tauri::{
    menu::{Menu, MenuItem, CheckMenuItem, MenuBuilder},
    tray::TrayIconBuilder,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;

use std::sync::{Arc, Mutex};

mod platform;
use platform::{
    MainModule,
    windows::Windows,
};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _ = app.handle().plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                None,
            ));

            let wf_module = Arc::new(Mutex::new(Windows::new()));
            wf_module.lock().unwrap().initialize();

            let wf_arc = Arc::clone(&wf_module);
            let enable = CheckMenuItem::with_id(
                app,
                "enable",
                "Enable",
                true,
                true,
                None::<&str>
            )?;
            let quit = MenuItem::with_id(
                app,
                "quit",
                "Quit",
                true,
                None::<&str>
            )?;
            let autolaunch = CheckMenuItem::with_id(
                app,
                "autolaunch",
                "AutoLaunch",
                true,
                app.autolaunch().is_enabled().unwrap(),
                None::<&str>
            )?;
            let menu = MenuBuilder::new(app.handle())
                .items(&[&enable, &quit])
                .separator()
                .item(&autolaunch)
                .build()?;
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
                        "autolaunch" => {
                            if autolaunch.is_checked().unwrap() {
                                let _ = app.autolaunch().enable();
                            }
                            else {
                                let _ = app.autolaunch().disable();
                            }
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
