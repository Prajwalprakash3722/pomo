use std::time::Duration;
use crate::timer::config::PpomoConfig;
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

    pub fn set_state(&mut self, new_state: PlayState, time_elapsed: Option<Duration>) {
        self.play_state = new_state;
    }
}
impl Timer for ResumablePomodoroTimer{

    fn tick(&mut self, time_elapsed_since: Duration) -> TimerResult<()> {
        // propagate ticks only if playing
        match self.play_state{
            PlayState::Playing => {
                self.timer.tick(time_elapsed_since)
            }
            PlayState::Paused | PlayState::Stopped => {
                Ok(())
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use std::time::Duration;
    use crate::timer::config::PpomoConfig;
    use crate::timer::resumable::ResumablePomodoroTimer;
    use crate::timer::timer::Timer;

    #[test]
    fn test_normal_play(){
        let mut timer = ResumablePomodoroTimer::new(PpomoConfig::default(),0);
        timer.tick(Duration::from_millis(100)).unwrap();

    }
}