use std::{process::Command, process::Stdio};
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

pub fn get_users(path:&str) -> Vec<String>{
    let users: Result<std::process::Output, std::io::Error> = Command::new("chntpw").arg("-l").arg(path).output();
    let users: String = match users {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => "Not Found".to_string(),
    };
    let mut nonadminusers:Vec<String> = vec![];
    let splittedlines: std::str::Split<'_, &str> = users.split("\n");
    for username in splittedlines{
        if username.contains("|") & !username.contains("-")  & !username.contains("ADMIN") & !username.contains("dis/lock"){
            let udata: Vec<&str> = username.split("|").collect::<Vec<_>>();
            nonadminusers.insert(0, udata[1].replace(" ", ""));
        }
    }
    return  nonadminusers;
}

pub fn administrator_privileges(userid:&str, path:&str){
    let mut chntpw = Command::new("chntpw")
    .arg("-u")
    .arg(["0x", &userid].concat())
    .arg(path)
    .stdin(Stdio::piped())
    .spawn()
    .unwrap();
    sleep(Duration::from_millis(200));
    if let Some(stdin) = chntpw.stdin.as_mut() {
        stdin.write_all(b"3\n").expect("Failed to write to stdin");
        sleep(Duration::from_millis(200));
        stdin.write_all(b"y\n").expect("Failed to write to stdin");
        sleep(Duration::from_millis(200));
        stdin.write_all(b"q\n").expect("Failed to write to stdin");
        sleep(Duration::from_millis(200));
        stdin.write_all(b"y\n").expect("Failed to write to stdin");
    }
    let output = chntpw
    .wait_with_output()
    .expect("Failed to read stdout");
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}