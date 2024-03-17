#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TimerStateError {
    InvalidState,
}

pub type TimerResult<T> = Result<T, TimerStateError>;