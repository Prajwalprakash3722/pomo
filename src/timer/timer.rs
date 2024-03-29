use std::time::Duration;
use crate::timer::config::PomoConfig;
use crate::timer::mode::PomodoroTimerMode;
use crate::timer::result::TimerResult;
use crate::timer::state::PomodoroTimer;

pub trait Timer{
    /// Update timer state, given that a specified timer has been spent since last tick.
    fn tick(&mut self, time_elapsed_since: Duration) -> TimerResult<()>;
    fn get_state(&self) -> PomodoroTimerMode;
    
    fn get_duration_left(&self) -> Duration;
    
    fn set_duration_left(&mut self, duration_left: Duration) -> TimerResult<()>;
    fn get_cycles_done(&self) -> u8;
    fn get_config(&self) -> PomoConfig;
}
