use std::time::Duration;
use crate::timer::config::PpomoConfig;
use crate::timer::mode::PomodoroTimerMode;
use crate::timer::result::TimerResult;
use crate::timer::state::PomodoroTimer;
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
    pub fn new(ppomo_config: PpomoConfig, cycles_done: u8) -> Self {
        let timer = PomodoroTimer::new(ppomo_config, cycles_done);
        let play_state = PlayState::Stopped;
        ResumablePomodoroTimer {
            timer,
            play_state,
        }
    }
    /// Set a new state, and optionally indicate how many ticks it has been since last call.
    pub fn set_state(&mut self, new_state: PlayState) {
        self.play_state = new_state;
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
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::timer::config::PpomoConfig;
    use crate::timer::resumable::{PlayState, ResumablePomodoroTimer};
    use crate::timer::timer::Timer;

    #[test]
    fn test_stopped_play() {
        let mut timer = ResumablePomodoroTimer::new(PpomoConfig::default(), 0);
        timer.tick(Duration::from_secs(100)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60), timer.get_duration_left());
    }

    #[test]
    fn test_normal_play() {
        let mut timer = ResumablePomodoroTimer::new(PpomoConfig::default(), 0);
        timer.set_state(PlayState::Playing);
        timer.tick(Duration::from_secs(100)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60 - 100), timer.get_duration_left());
    }

    #[test]
    fn test_paused_play() {
        let mut timer = ResumablePomodoroTimer::new(PpomoConfig::default(), 0);
        timer.set_state(PlayState::Paused);
        timer.tick(Duration::from_secs(100)).unwrap();
        assert_eq!(Duration::from_secs(25 * 60), timer.get_duration_left());
    }
}