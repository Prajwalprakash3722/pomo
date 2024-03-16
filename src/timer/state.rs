use std::time::Duration;
use crate::timer::config::PpomoConfig;
use crate::timer::mode::TimerMode;

#[derive(Debug)]
pub struct TimerState {
    config: PpomoConfig,
    millis_left_in_state: Duration,
    mode: TimerMode,
    cycles_done: u8,
}

impl TimerState {
    /// Update timer state, given that a specified timer has been spent since last tick.
    pub fn tick(&mut self, time_elapsed_since: Duration) {
        let sub_duration = self.millis_left_in_state.checked_sub(time_elapsed_since);
        match sub_duration {
            None => {
                let carry_over = time_elapsed_since - self.millis_left_in_state;

                self.check_and_increment_cycle();
                //no more time. Tick tick change
                self.change_state_and_reset_time();

                self.tick(carry_over);
            }
            Some(sub_duration) => {
                self.millis_left_in_state = sub_duration
            }
        }
    }

    fn check_and_increment_cycle(&mut self) {
        if self.mode == TimerMode::Break || self.mode == TimerMode::LongBreak {
            self.cycles_done += 1;
        }
    }
    fn change_state_and_reset_time(&mut self) {
        self.mode = self.mode.next(self.cycles_done, self.config.cycles_per_long_break);
        self.millis_left_in_state = self.mode.get_new_time_left_millis_for_state(&self.config)
    }
    fn get_state(&self)->TimerMode{
        self.mode.clone()
    }
    fn get_duration_left(&self)->Duration{self.millis_left_in_state}
    fn get_cycles_done(&self)->u8{self.cycles_done}
}


#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::timer::config::PpomoConfig;
    use crate::timer::mode::TimerMode::{Break, Work};
    use crate::timer::state::TimerState;

    #[test]
    pub fn regular_tick() {
        let ppomoconfig = PpomoConfig::default();
        let mut timer_state = TimerState {
            millis_left_in_state: Duration::from_millis(1000),
            mode: Break,
            cycles_done: 0,
            config: ppomoconfig.clone(),
        };
        timer_state.tick(Duration::from_millis(250));
        assert_eq!(Break, timer_state.mode);
        assert_eq!(Duration::from_millis(750), timer_state.millis_left_in_state);
        assert_eq!(0,timer_state.cycles_done);
    }

    #[test]
    pub fn sucessive_ticks() {
        let ppomoconfig = PpomoConfig::default();
        let mut timer_state = TimerState {
            millis_left_in_state: Duration::from_millis(1004),
            mode: Break,
            cycles_done: 0,
            config: ppomoconfig.clone(),
        };
        for _i in 0..4{
            timer_state.tick(Duration::from_millis(250));
        }
        assert_eq!(Break, timer_state.mode);
        assert_eq!(Duration::from_millis(4), timer_state.millis_left_in_state);
        assert_eq!(0,timer_state.cycles_done);
    }


    #[test]
    pub fn tick_to_zero() {
        let ppomo_config = PpomoConfig::default();
        let mut timer_state = TimerState {
            millis_left_in_state: Duration::from_millis(1000),
            mode: Break,
            cycles_done: 0,
            config: ppomo_config.clone(),
        };
        timer_state.tick(Duration::from_millis(1000));
        assert_eq!(Break, timer_state.mode);
        assert_eq!(Duration::new(0, 0), timer_state.millis_left_in_state);
    }


    #[test]
    pub fn tick_and_move() {
        let ppomoconfig = PpomoConfig::default();
        let mut timer_state = TimerState {
            millis_left_in_state: Duration::from_millis(1000),
            mode: Break,
            cycles_done: 0,
            config: ppomoconfig.clone(),
        };
        timer_state.tick(Duration::from_millis(1001));
        assert_eq!(Work, timer_state.mode);
        assert_eq!(Duration::from_millis(25 * 60 * 1000 - 1), timer_state.millis_left_in_state);
    }
}