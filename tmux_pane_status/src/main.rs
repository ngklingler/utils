extern crate subprocess;

use std::{collections::HashMap, env, path};

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

fn cd(new_dir: &String) {
    let root = path::Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
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
    if split.len() < 2 { return result; } // no file mod lines
    // TODO probably faster not to use a hashmap
    let mut status_count = HashMap::new();
    for line in &split[1..] {
        let index = if line.starts_with(" ") { 1 } else { 0 };
        let msg = line.split(' ').collect::<Vec<&str>>()[index];
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
    // Args should be a directory to get git status for and $HOME
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Util Error");
    }
    let pwd = capture("pwd");
    // TODO find a more efficient way to get rid of the \n at the end
    let pwd = pwd.split('\n').collect::<Vec<&str>>()[0];
    cd(&args[1]);
    // replace home dir part with ~
    let dir = if args[1].starts_with(&args[2]) {
        format!("~{}", &args[1][args[2].len()..])
    } else {
       args[1].clone()
    };
    println!("{} {}", dir, parse_git_status(capture("git status -s -b")));
    cd(&pwd.to_string());
}
