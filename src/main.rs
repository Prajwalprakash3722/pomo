use std::process::exit;

use clap::Parser;

mod args;

fn main() {
    let mut args = args::PomodoroArgs::parse();

    args.duration = args.duration.max(1);
    args.break_duration = args.break_duration.max(1);
    args.long_break_duration = args.long_break_duration.max(1);
    args.cycles = args.cycles.max(1);

    if let Err(err) = run_pomodoro(args) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

fn run_pomodoro(args: args::PomodoroArgs) -> Result<(), String> {
    println!("Pomodoro timer starting! Work periods will be {} minutes long, followed by short breaks of {} minutes. After completing {} work periods, you'll have a longer break of {} minutes.", args.duration, args.break_duration, args.cycles, args.long_break_duration);
    Ok(())
}
