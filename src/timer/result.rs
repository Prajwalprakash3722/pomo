#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimerStateError {
    InvalidState,
}

pub type TimerResult<T> = Result<T, TimerStateError>;