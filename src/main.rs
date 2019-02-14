extern crate reqwest;
use std::process::Command;

fn passed_check()
{
    println!("OK");
}

fn failed_check()
{
    let mut list_dir = Command::new("ls");
    list_dir.status().expect("process failed to execute");
}

fn heartbeat()
{
    let response = reqwest::get("https://httpbin.org/status/200").expect("Failed to send request");
    if response.status() == 200 {
        passed_check();
    } else {
        failed_check();
    }
}

fn main()
{
    loop {
        heartbeat()
    }
}
