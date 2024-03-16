use std::time::Duration;
use crate::timer::config::PpomoConfig;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TimerMode {
    Work,
    Break,
    LongBreak,
}

impl Default for TimerMode {
    fn default() -> Self {
        Self::Work
    }
}

impl TimerMode {
    /// Next
    pub fn next(&self, works_done: u8, works_per_long_break: u8) -> TimerMode {
        match self {
            TimerMode::Work => {
                if works_done % works_per_long_break == 0 {
                    TimerMode::LongBreak
                } else {
                    TimerMode::Break
                }
            }
            TimerMode::Break | TimerMode::LongBreak => {
                TimerMode::Work
            }
        }
    }
    const TIMER_MINUTES_TO_MILLIS: u64 = 60;
    pub fn get_new_time_left_millis_for_state(&self, state: &PpomoConfig) -> Duration {
        let new_time_minutes = match self {
            TimerMode::Work => { state.work_duration_minutes }
            TimerMode::Break => { state.break_duration_minutes }
            TimerMode::LongBreak => { state.long_break_duration_minutes }
        } as u64;
        Duration::from_secs(new_time_minutes * 60)
    }
}

#[cfg(test)]
mod test {
    use crate::timer::mode::TimerMode::*;

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
