use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

use std::thread;
use std::time::Duration;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let test = MenuItem::with_id(app, "test", "Test", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&test, &quit])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "test" => {
                        println!("menu item show clicked");
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id());
                    }
                })
                .build(app)?;

            thread::spawn(|| {
                loop {
                    println!("1sec...");
                    thread::sleep(Duration::from_secs(1));
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
