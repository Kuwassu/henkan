use tray_icon::{TrayIconBuilder, Icon};

fn main() {
    let icon = Icon::from_path("kuwassu256.ico", Some((32,32))).unwrap();
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("henkan")
        .with_icon(icon)
        .build()
        .unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }

}
