use std::time::Duration;
use crate::timer::config::PomoConfig;
use crate::timer::mode::PomodoroTimerMode;
use crate::timer::result::TimerResult;
use crate::timer::pomo::PomodoroTimer;
use crate::timer::timer::Timer;

enum PlayState {
    Playing,
    Paused,
    Stopped,
}

/// A ppomodoro timer that also supports updating
struct ResumablePomodoroTimer {
    timer: PomodoroTimer,
    play_state: PlayState,
}

impl ResumablePomodoroTimer {
    /// Create a new stopped Pomodoro Timer
    pub fn new(ppomo_config: PomoConfig, cycles_done: u8) -> Self {
        let timer = PomodoroTimer::new(ppomo_config, cycles_done);
        let play_state = PlayState::Stopped;
        ResumablePomodoroTimer {
            timer,
            play_state,
        }
    }
    /// Set a new state, and optionally indicate how many ticks it has been since last call.
    pub fn set_state(&mut self, new_state: PlayState) ->TimerResult<()> {
        if let PlayState::Stopped = new_state{
            let config = self.get_config();
            self.timer.set_duration_left( self.timer.get_state().get_new_time_left_millis_for_state(&config))?;
        }
        self.play_state = new_state;
        Ok(())
    }
}

impl Timer for ResumablePomodoroTimer {
    fn tick(&mut self, time_elapsed_since: Duration) -> TimerResult<()> {
        // propagate ticks only if playing
        match self.play_state {
            PlayState::Playing => {
                self.timer.tick(time_elapsed_since)
            }
            PlayState::Paused | PlayState::Stopped => {
                Ok(())
            }
        }
    }

    fn get_state(&self) -> PomodoroTimerMode {
        self.timer.get_state()
    }

    fn get_duration_left(&self) -> Duration {
        self.timer.get_duration_left()
    }

    fn set_duration_left(&mut self, duration_left: Duration) -> TimerResult<()> {
        self.timer.set_duration_left(duration_left)
    }

    fn get_cycles_done(&self) -> u8 {
        self.timer.get_cycles_done()
    }

    fn get_config(&self) -> PomoConfig {
        self.timer.get_config()
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::timer::config::PomoConfig;
    use crate::timer::resumable_pomo::{PlayState, ResumablePomodoroTimer};
    use crate::timer::timer::Timer;

    #[test]
    fn test_stopped_play() {
        let mut timer = ResumablePomodoroTimer::new(PomoConfig::default(), 0);
        timer.tick(Duration::from_secs(100)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60), timer.get_duration_left());
    }

    #[test]
    fn test_normal_play() {
        let mut timer = ResumablePomodoroTimer::new(PomoConfig::default(), 0);
        timer.set_state(PlayState::Playing).expect("Should be Playing now");
        timer.tick(Duration::from_secs(100)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60 - 100), timer.get_duration_left());
    }

    #[test]
    fn test_paused_play() {
        let mut timer = ResumablePomodoroTimer::new(PomoConfig::default(), 0);
        timer.set_state(PlayState::Paused).expect("Should be paused now");
        timer.tick(Duration::from_secs(100)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60), timer.get_duration_left());
    }

    #[test]
    fn test_stop_resets_time_left() {
        let mut timer = ResumablePomodoroTimer::new(PomoConfig::default(), 0);
        timer.set_state(PlayState::Playing).expect("Should be playing now");
        timer.tick(Duration::from_secs(100)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60 - 100), timer.get_duration_left());
        timer.set_state(PlayState::Stopped).expect("Should be stopped now");
        assert_eq!(Duration::from_secs(25 * 60), timer.get_duration_left());
        timer.tick(Duration::from_millis(100000)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60), timer.get_duration_left());
    }
}