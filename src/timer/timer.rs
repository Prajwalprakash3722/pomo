use std::time::Duration;
use crate::timer::result::TimerResult;

pub trait Timer{
    /// Update timer state, given that a specified timer has been spent since last tick.
    fn tick(&mut self, time_elapsed_since: Duration) -> TimerResult<()>;
}
