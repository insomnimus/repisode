use std::process;

use repisode::cmd::Cmd;

fn main() {
    if let Err(e) = Cmd::from_args().run() {
        eprintln!("error: {}", e);
        process::exit(2);
    }
}
