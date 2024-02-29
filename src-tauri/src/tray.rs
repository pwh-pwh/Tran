use std::sync::atomic::Ordering;

use anyhow::Result;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, Submenu},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};

use crate::config::{self, MODE};

/// 初始化托盘菜单
///
/// Initialize tray menu
pub fn init(app: &AppHandle) -> Result<()> {
    let menu = menu(app)?;
    let _ = TrayIconBuilder::with_id("menu")
        .tooltip("Tran")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(handler)
        .build(app);
    Ok(())
}

fn menu(handle: &AppHandle) -> Result<Menu<Wry>> {
    let flag = MODE.load(Ordering::SeqCst);
    let github = MenuItem::with_id(handle, "github", "GitHub", true, None::<&str>)
        .expect("Failed to create menu item github");
    let mirror = CheckMenuItem::with_id(handle, "mirror", "Mirror", true, flag, None::<&str>)
        .expect("Failed to create menu item mirror");
    let google = CheckMenuItem::with_id(handle, "google", "Google", true, !flag, None::<&str>)
        .expect("Failed to create menu item google");
    let mode = Submenu::with_items(handle, "Mode", true, &[&mirror, &google])
        .expect("Failed to create submenu item mod.");
    let exit = MenuItem::with_id(handle, "exit", "Exit", true, None::<&str>)
        .expect("Failed to create menu item exit");
    Menu::with_items(handle, &[&github, &mode, &exit])
        .map_err(|_| anyhow::anyhow!("Failed to create menu"))
}

fn fresh(app: &AppHandle) {
    let _ = app.tray().unwrap().set_menu(Some(menu(app).unwrap()));
}

fn handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "github" => {
            let _ = open::that("https://github.com/Borber/Tran");
        }
        "mirror" => {
            config::mode(true);
            fresh(app);
        }
        "google" => {
            config::mode(false);
            fresh(app);
        }
        "exit" => {
            let panel = app.get_webview_window("panel").unwrap();
            let _ = panel.hide();
            app.exit(0)
        }
        _ => {}
    }
}
