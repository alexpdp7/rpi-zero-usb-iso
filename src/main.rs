use std::io::Write as _;
use std::os::linux::fs::MetadataExt;

use clap::Parser;
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

fn iso_directory() -> std::path::PathBuf {
    std::env::home_dir().unwrap().join("isos")
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Setup,
    Iso,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Setup => {
            let requires_reboot = enable_usb();
            std::fs::create_dir_all(iso_directory()).unwrap();
            eprintln!("Place isos at {:?}", iso_directory());
            assert!(
                std::process::Command::new("loginctl")
                    .arg("enable-linger")
                    .status()
                    .unwrap()
                    .success()
            );
            let systemd_user_directory = std::env::home_dir()
                .unwrap()
                .join(".config")
                .join("systemd")
                .join("user");
            std::fs::create_dir_all(&systemd_user_directory).unwrap();
            std::fs::File::create(systemd_user_directory.join("rpi-zero-usb-iso.service"))
                .unwrap()
                .write_all(include_bytes!("rpi-zero-usb-iso.service"))
                .unwrap();
            assert!(
                std::process::Command::new("systemctl")
                    .args(["--user", "enable", "rpi-zero-usb-iso.service"])
                    .status()
                    .unwrap()
                    .success()
            );
            if requires_reboot {
                eprintln!("Reboot");
            }
        }
        Commands::Iso => {
            let mut isos = iso_directory()
                .read_dir()
                .unwrap()
                .flatten()
                .collect::<Vec<_>>();
            isos.sort_by_key(|de| de.metadata().unwrap().st_mtime());
            eprintln!("Found isos by date {isos:?}");
            let iso = isos.last().expect("to have one iso");
            eprintln!("Using {iso:?}");
            assert!(
                std::process::Command::new("sudo")
                    .args([
                        "modprobe",
                        "g_mass_storage",
                        &format!("file={}", iso.path().to_str().unwrap()),
                    ])
                    .status()
                    .unwrap()
                    .success()
            );
        }
    }
}
