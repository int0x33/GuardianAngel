extern crate reqwest;
use std::io::Read;
use std::process::Command;

fn heartbeat()
{
    let mut response = reqwest::get("https://httpbin.org/status/500").expect("Failed to send request");
    if response.status() == 200 {
        println!("{}", response.status());
        let mut buf = String::new();
        response.read_to_string(&mut buf).expect("Failed to read response");
        println!("{}", buf);
    } else {
        let mut list_dir = Command::new("ls");
        list_dir.status().expect("process failed to execute");
    }
}

fn main()
{
    loop {
        heartbeat()
    }
}
