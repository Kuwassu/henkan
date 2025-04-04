use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem} ,Icon};

fn main() {
    let menu = Menu::new();
    let _ = menu.append(&MenuItem::new("終了", true, None));
    let _ = menu.append(&MenuItem::new("設定", false, None));

    let icon = Icon::from_path("kuwassu256.ico", Some((32,32))).unwrap();
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("henkan")
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
