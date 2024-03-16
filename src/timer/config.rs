use crate::args::PomodoroArgs;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PpomoConfig {
    work_duration_minutes: u8,
    break_duration_minutes: u8,
    long_break_duration_minutes: u8,
    cycles: u8,
}

impl From<PomodoroArgs> for PpomoConfig{
    fn from(value: PomodoroArgs) -> Self {
        Self{
            work_duration_minutes: value.duration,
            break_duration_minutes: value.break_duration,
            long_break_duration_minutes: value.long_break_duration,
            cycles: value.cycles
        }
    }
}

