use std::time::Duration;
use crate::timer::config::PomoConfig;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PomodoroTimerMode {
    Work,
    Break,
    LongBreak,
}

impl Default for PomodoroTimerMode {
    fn default() -> Self {
        Self::Work
    }
}

impl PomodoroTimerMode {
    /// Next
    pub fn next(&self, works_done: u8, works_per_long_break: u8) -> PomodoroTimerMode {
        match self {
            PomodoroTimerMode::Work => {
                if works_done % works_per_long_break == 0 {
                    PomodoroTimerMode::LongBreak
                } else {
                    PomodoroTimerMode::Break
                }
            }
            PomodoroTimerMode::Break | PomodoroTimerMode::LongBreak => {
                PomodoroTimerMode::Work
            }
        }
    }
    const TIMER_MINUTES_IN_SEC: u64 = 60;
    pub fn get_new_time_left_millis_for_state(&self, state: &PomoConfig) -> Duration {
        let new_time_minutes = match self {
            PomodoroTimerMode::Work => { state.work_duration_minutes }
            PomodoroTimerMode::Break => { state.break_duration_minutes }
            PomodoroTimerMode::LongBreak => { state.long_break_duration_minutes }
        } as u64;
        Duration::from_secs(new_time_minutes * Self::TIMER_MINUTES_IN_SEC)
    }
}

#[cfg(test)]
mod test {
    use crate::timer::mode::PomodoroTimerMode::*;

    #[test]
    pub fn test_next_for_break() {
        let curr_state = Break;
        assert_eq!(Work, curr_state.next(0, 5))
    }

    #[test]
    pub fn test_next_for_long_break() {
        let curr_state = LongBreak;
        assert_eq!(Work, curr_state.next(1, 4))
    }

    #[test]
    pub fn test_next_for_work_to_long_break_for_divisible() {
        let curr_state = Work;
        assert_eq!(LongBreak, curr_state.next(4, 4))
    }

    #[test]
    pub fn test_next_for_work_to_long_break_for_multiple_works_done() {
        let curr_state = Work;
        assert_eq!(LongBreak, curr_state.next(12, 4))
    }

    #[test]
    pub fn test_next_for_work_to_break_for_indivisible_works_done() {
        let curr_state = Work;
        assert_eq!(Break, curr_state.next(3, 4))
    }
}
