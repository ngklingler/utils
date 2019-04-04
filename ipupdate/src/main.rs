use std::process::{Command, Stdio};
use std::{str, fs, env};

fn command(command: &str) -> String {
    let split: Vec<&str> = command.split(' ').collect();
    let output = match split.len() {
        1 => Command::new(split[0])
                .stdout(Stdio::piped())
                .output()
                .unwrap(),
        _ => Command::new(split[0])
                .args(&split[1..])
                .stdout(Stdio::piped())
                .output()
                .unwrap()
    };
    String::from(str::from_utf8(&output.stdout).unwrap())
}

fn get_current_ip(device: &str) -> String {
    let ifconfig = command("ifconfig");
    let mut next_line = false;
    let mut ip = String::new();
    for line in ifconfig.split('\n') {
        if line.starts_with(device) {
            next_line = true;
            continue;
        }
        if next_line {
            // TODO replace with regex
            let mut count = 0;
            for c in line.chars() {
                if count > 0 {
                    if c == ' ' {
                        if count == 2 {
                            return ip;
                        } else {
                        count += 1;
                        }
                    } else {
                        ip.push(c);
                    }
                } else if c == 't' {
                    count = 1;
                }
            }
        }
    }
    return ip;
}

fn read_last_ip(filename: &str) -> Option<String> {
    let read = fs::read_to_string(filename);
    match read {
        Ok(i) => return Some(i),
        Err(_e) => return None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Error: not enough args, must supply machine name and device you want IP address for.");
    }
    let machine = &args[1];
    let device = &args[2];
    
    let _ = command("git pull");
    let last = read_last_ip(machine);
    let current = get_current_ip(device);
    if let Some(i) = last {
        if i == current {
            return;
        }
    }
    fs::write(machine, &current).unwrap();
    // TODO commit and push
    let _ = command("git add .");
    let commit = format!("git commit -m \"update {} to {}\"", machine, current);
    let _ = command(&commit);
    let _ = command("git pull");
    let _ = command("git push");
}
