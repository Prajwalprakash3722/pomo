use std::time::Duration;
use crate::timer::mode::PomodoroTimerMode;
use crate::timer::result::TimerResult;
use crate::timer::state::PomodoroTimer;

pub trait Timer{
    /// Update timer state, given that a specified timer has been spent since last tick.
    fn tick(&mut self, time_elapsed_since: Duration) -> TimerResult<()>;
    fn get_state(&self) -> PomodoroTimerMode;
    
    fn get_duration_left(&self) -> Duration;
    
}
