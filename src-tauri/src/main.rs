#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api::shell::open, AppHandle, CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

const links: [(&str, &str, &str); 2] = [
    // social links
    ("open-github-personal", "Personal","https://github.com/darVoid/"),
    ("open-github-work", "Duque's Cult 🍆","https://google.com/"),
];

fn main() {
    let sub_menu_github = {
        let mut menu = SystemTrayMenu::new();
        for (id, label, _url) in
            links.iter().filter(|(id, label, _url)| {
                id.starts_with("open-github")
            })
        {
            menu = menu.add_item(CustomMenuItem::new(
                id.to_string(),
                label.to_string(),
            ));
        }

        SystemTraySubmenu::new("GitHub", menu)
    };
    let tray_menu = SystemTrayMenu::new()
        
        .add_submenu(sub_menu_github)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(
            "visibility-toggle".to_string(),
            "Hide",
        )).add_item(CustomMenuItem::new(
            "quit".to_string(),
            "Quit",
        ));

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(on_system_tray_event)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested {
                api, ..
            } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn onNewPullRequest(app: &AppHandle){
    app.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../icons/tray.png").to_vec())).unwrap();
}

fn on_system_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle =
                app.tray_handle().get_item(&id);
            dbg!(&id);
            match id.as_str() {
                "visibility-toggle" => {
                    let window =
                        app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(true) => {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        },
                        Ok(false) => {
                            window.show();
                            item_handle.set_title("Hide").unwrap();
                        },
                        Err(e) => unimplemented!("what kind of errors happen here?"),
                    }
                }
                "quit" => app.exit(0),
                s if s.starts_with("open-") => {
                    if let Some(link) = links
                        .iter()
                        .find(|(id, ..)| id == &s)
                    {
                        open(
                            &app.shell_scope(),
                            link.2,
                            None,
                        )
                        .unwrap();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}