extern crate notify;
use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::env;
use std::process::Command;
use std::sync::mpsc;

fn make_command(cmd: &Vec<String>) -> Result<std::process::Child, String> {
  match Command::new(cmd.first().unwrap()).args(&cmd[1..]).spawn() {
    std::io::Result::Ok(v) => Ok(v),
    std::io::Result::Err(e) => Err(format!("Error running command '{:?}':\n{}", cmd, e)),
  }
}

fn parse_args() -> Result<(Vec<String>, Vec<String>), String> {
  let mut args = env::args();
  args.next();
  let args: Vec<String> = args.collect();
  match args.iter().position(|a| a == "--") {
    Some(ix) if ix > 0 => Ok((Vec::from(&args[..ix]), Vec::from(&args[ix + 1..]))),
    _                  => Err(format!(
      "{}\n  {}\n",
      "Insufficient arguments. Syntax:", "watsch FILE1 [FILE2, ...] -- COMMAND [ARGUMENTS]",
    )),
  }
}

fn run() -> Result<(), String> {
  let (paths, command) = parse_args()?;
  let (watch_tx, watch_rx) = mpsc::channel();

  let mut watcher = notify::watcher(watch_tx, std::time::Duration::from_millis(200)).unwrap();
  for path in paths {
    watcher.watch(path, RecursiveMode::Recursive).unwrap();
  }

  let cmd = command;
  let mut child_process = make_command(&cmd).unwrap();
  loop {
    match watch_rx.recv() {
      Ok(event) => {
          match event {
            // The set of events that correspond to a command restart
            DebouncedEvent::Create(_) | DebouncedEvent::Write(_) => {
              child_process.kill().unwrap();
              match child_process.kill() {
                Ok(_) => {},
                Err(_) => {},
              };
              match child_process.wait() {
                Ok(_) => {},
                Err(_) => {},
              };
              child_process = make_command(&cmd)?;
            }
            _ => {},
          }
      }
      Err(e) => eprintln!("watch error: {:?}", e),
    }
  }
}

fn main() {
  match run() {
    Ok(_) => {}
    Err(e) => println!("{}", e),
  }
}
