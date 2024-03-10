use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "A pomodoro timer CLI app")]
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
}