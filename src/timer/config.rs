use crate::args::PomodoroArgs;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PomoConfig {
    pub work_duration_minutes: u8,
    pub break_duration_minutes: u8,
    pub long_break_duration_minutes: u8,
    pub cycles_to_do: u8,
    pub cycles_per_long_break: u8,
}

impl Default for PomoConfig {
    /// A bog-standard pomodoro timing configuration.
    /// Mainly used for testing.
    fn default() -> Self {
        Self{
            work_duration_minutes: 25,
            break_duration_minutes: 5,
            long_break_duration_minutes: 15,
            cycles_to_do: 4,
            cycles_per_long_break: 4,
        }
    }
}

impl From<PomodoroArgs> for PomoConfig {
    fn from(value: PomodoroArgs) -> Self {
        Self{
            work_duration_minutes: value.duration,
            break_duration_minutes: value.break_duration,
            long_break_duration_minutes: value.long_break_duration,
            cycles_to_do: value.cycles,
            // FIXME: Allow for customization
            cycles_per_long_break: 4
        }
    }
}

