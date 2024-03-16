use std::time::Duration;
use crate::timer::mode::TimerMode;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PlayState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub struct TimerState {
    pub millis_left_in_state: Duration,
    pub mode: TimerMode,
    pub state: PlayState,
    pub cycles_done: u8,
}

impl TimerState {
    fn tick(&mut self, time_elapsed_since: Duration) {
        let sub_duration = self.millis_left_in_state.checked_sub(time_elapsed_since);
        match sub_duration {
            None => { 
                //no more time. Tick tick change
            }
            Some(sub_duration) => { 
                self.millis_left_in_state = sub_duration
            },
        }
    }
}


#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::timer::mode::TimerMode::Break;
    use crate::timer::state::PlayState::Playing;
    use crate::timer::state::TimerState;

    #[test]
    pub fn tick_1000() {
        let mut timer_state = TimerState {
            millis_left_in_state: Duration::from_millis(1000),
            state: Playing,
            mode: Break,
            cycles_done: 0
        };
        timer_state.tick(Duration::from_millis(1000));
        assert!(timer_state.millis_left_in_state.is_zero());
    }
}