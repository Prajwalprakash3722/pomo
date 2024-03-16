use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name= "pomo",
    author = "prajwal@devcoffee.me, atreya@kekvrose.me",
    version = "0.1",
    about = "A pomodoro timer CLI app"
)]
pub struct PomodoroArgs {
    #[clap(
        short = 'd',
        long = "duration",
        default_value_t = 25,
        help = "Sets the duration of a work period in minutes"
    )]
    pub duration: u8,

    #[clap(
        short = 'b',
        long = "break",
        default_value_t = 5,
        help = "Sets the duration of a short break in minutes"
    )]
    pub break_duration: u8,

    #[clap(
        short = 'l',
        long = "long-break",
        default_value_t = 15,
        help = "Sets the duration of a long break in minutes after a set number of cycles"
    )]
    pub long_break_duration: u8,

    #[clap(
        short = 'c',
        long = "cycles",
        default_value_t = 4,
        help = "Sets the number of work periods before a long break"
    )]
    pub cycles: u8,

    #[clap(
        short = 'n',
        long = "name",
        help = "Sets a name for the current work period"
    )]
    pub name: Option<String>,

    #[clap(
        short = 'v',
        long = "visual",
        default_value_t = false,
        help = "Provides more detailed output during the pomodoro session, such as time remaining and progress"
    )]
    pub visual: bool,
}
