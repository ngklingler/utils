extern crate subprocess;

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

fn create_or_attach_tmux() {
    let sessions = capture("tmux ls -F '#{session_attached}#{session_name}'");
    for s in sessions.split('\n') {
        if s.len() > 2 {
            if &s[1..2] == "0" {
                println!("tmux a -t {}", &s[2..s.len() -1]);
                return;
            }
        }
    }
    println!("tmux new");
}

fn main() {
    create_or_attach_tmux();
}
