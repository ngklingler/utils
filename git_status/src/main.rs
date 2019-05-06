extern crate subprocess;

use std::collections::HashMap;

fn capture(command: &str) -> String {
    let split = command.split(' ').collect::<Vec<&str>>();
    let out = subprocess::Exec::cmd(&split[0])
        .args(&split[1..])
        .stdout(subprocess::Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();
    out
}

fn parse_git_status(output: String) -> String {
    if output == "" || output.starts_with("fatal") { 
        return String::from(""); 
    }
    let split = output.split('\n').collect::<Vec<&str>>();
    // TODO fix this below
    let mut result = split[0].split('.')
                             .collect::<Vec<&str>>()[0][3..]
                             .to_string();
    if split.len() < 2 { return result; }
    let mut status_count = HashMap::new();
    for line in &split[1..] {
        let msg = line.split(' ').collect::<Vec<&str>>()[0];
        let count = status_count.entry(msg).or_insert(0);
        *count += 1;
    }
    result.push(' ');
    // TODO should this be sorted?
    for (status, count) in status_count {
        if status == "" { continue; }
        result = format!("{}{}{}", result, count, status);
    }
    result
}

fn main() {
    println!("{}", parse_git_status(capture("git status -s -b")));
}
