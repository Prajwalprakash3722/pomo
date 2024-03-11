use args::PomodoroArgs;
use clap::Parser;

mod args;
mod ui;

fn main() {
    let mut args = args::PomodoroArgs::parse();

    args.duration = args.duration.max(1);
    args.break_duration = args.break_duration.max(1);
    args.long_break_duration = args.long_break_duration.max(1);
    args.cycles = args.cycles.max(1);

    let name = match args.name {
        Some(name) => name,
        None => "default".to_string(),
    };

    args.name = Some(name);

    if let Err(err) = run_pomodoro(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run_pomodoro(args: PomodoroArgs) -> Result<(), String> {
    let msg = format!("Pomodoro timer starting! Work periods named {} will be {} minutes long, followed by short breaks of {} minutes. After completing {} work periods, you'll have a longer break of {} minutes.", args.name.unwrap(), args.duration, args.break_duration, args.cycles, args.long_break_duration);
    if args.visual {
        ui::render_ui().expect("Failed to Render UI")
    } else {
        println!("{}", msg)
    }
    Ok(())
}
