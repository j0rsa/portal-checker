extern crate reqwest;
extern crate tokio;

use std::time::Duration;
use tokio::time::sleep;
use reqwest::Error;
use std::process::Command;
use tray_item::{IconSource, TIError, TrayItem};

#[tokio::main]
async fn main() -> Result<(), TIError> {
    let mut tray = TrayItem::new("Portal checker", IconSource::Resource("default")).unwrap();

    // tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Request TOTP", || {
        Command::new("cmd")
            .args(&["/C", "start", "http://portal"])
            .spawn()
            .expect("Failed to open portal");
    }).unwrap();

    let mut inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
    loop {
        match check_portal().await {
            Ok(status) => {
                match status {
                    PortalStatus::Ok => tray.set_icon(IconSource::Resource("positive")),
                    PortalStatus::TOTP => tray.set_icon(IconSource::Resource("default")),
                    PortalStatus::Error => tray.set_icon(IconSource::Resource("negative")),
                };
            }
            Err(_) => tray.set_icon(IconSource::Resource("default")).unwrap(),
        }
        sleep(Duration::from_secs(10)).await;
    }
    Ok(())
}

async fn check_portal() -> Result<PortalStatus, Error> {
    let resp = reqwest::get("http://portal").await?;
    let body = resp.text().await?;
    if body.contains("TOTP") {
        Ok(PortalStatus::TOTP)
    } else {
        Ok(PortalStatus::Ok)
    }
}

enum PortalStatus {
    Ok,
    TOTP,
    Error,
}