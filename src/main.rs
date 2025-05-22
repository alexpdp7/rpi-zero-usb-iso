use std::io::Write as _;

use tempfile::NamedTempFile;

static DT_OVERLAY: &str = "dtoverlay=dwc2\n";
static CONFIG_TXT_PATH: &str = "/boot/firmware/config.txt";

fn enable_usb() -> bool {
    let config_txt_path = std::path::Path::new(CONFIG_TXT_PATH);
    let mut config_txt = std::fs::read_to_string(config_txt_path).unwrap();
    if config_txt.ends_with(DT_OVERLAY) {
        return false;
    }
    config_txt.push_str(DT_OVERLAY);
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(&config_txt.into_bytes()).unwrap();
    assert!(
        std::process::Command::new("sudo")
            .args(["mv", file.path().to_str().unwrap(), CONFIG_TXT_PATH])
            .status()
            .unwrap()
            .success()
    );
    true
}

fn main() {
    let requires_reboot = enable_usb();
    if requires_reboot {
        eprintln!("Reboot");
    }
}
