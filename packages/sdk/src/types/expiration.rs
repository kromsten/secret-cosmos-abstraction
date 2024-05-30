use cosmwasm_schema::cw_serde;
use cosmwasm_std::{BlockInfo, StdError, StdResult, Timestamp};
use std::ops::{Add, Mul};
use std::cmp::Ordering;
use std::fmt;

/// Expiration represents a point in time when some event happens.
/// It can compare with a BlockInfo and will return is_expired() == true
/// once the condition is hit (and for every block in the future)
#[cw_serde]
#[derive(Copy)]
pub enum Expiration {
    /// AtHeight will expire when `env.block.height` >= height
    AtHeight(u64),
    /// AtTime will expire when `env.block.time` >= time
    AtTime(Timestamp),
    /// Never will never expire. Used to express the empty variant
    Never {},
}


impl fmt::Display for Expiration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expiration::AtHeight(height) => write!(f, "expiration height: {}", height),
            Expiration::AtTime(time) => write!(f, "expiration time: {}", time),
            Expiration::Never {} => write!(f, "expiration: never"),
        }
    }
}

/// The default (empty value) is to never expire
impl Default for Expiration {
    fn default() -> Self {
        Expiration::Never {}
    }
}

impl Expiration {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        match self {
            Expiration::AtHeight(height) => block.height >= *height,
            Expiration::AtTime(time) => block.time >= *time,
            Expiration::Never {} => false,
        }
    }
}

impl Add<Duration> for Expiration {
    type Output = StdResult<Expiration>;

    fn add(self, duration: Duration) -> StdResult<Expiration> {
        match (self, duration) {
            (Expiration::AtTime(t), Duration::Time(delta)) => {
                Ok(Expiration::AtTime(t.plus_seconds(delta)))
            }
            (Expiration::AtHeight(h), Duration::Height(delta)) => {
                Ok(Expiration::AtHeight(h + delta))
            }
            (Expiration::Never {}, _) => Ok(Expiration::Never {}),
            _ => Err(StdError::generic_err("Cannot add height and time")),
        }
    }
}

// TODO: does this make sense? do we get expected info/error when None is returned???
impl PartialOrd for Expiration {
    fn partial_cmp(&self, other: &Expiration) -> Option<Ordering> {
        match (self, other) {
            // compare if both height or both time
            (Expiration::AtHeight(h1), Expiration::AtHeight(h2)) => Some(h1.cmp(h2)),
            (Expiration::AtTime(t1), Expiration::AtTime(t2)) => Some(t1.cmp(t2)),
            // if at least one is never, we can compare with anything
            (Expiration::Never {}, Expiration::Never {}) => Some(Ordering::Equal),
            (Expiration::Never {}, _) => Some(Ordering::Greater),
            (_, Expiration::Never {}) => Some(Ordering::Less),
            // if they are mis-matched finite ends, no compare possible
            _ => None,
        }
    }
}

pub const HOUR: Duration = Duration::Time(60 * 60);
pub const DAY: Duration = Duration::Time(24 * 60 * 60);
pub const WEEK: Duration = Duration::Time(7 * 24 * 60 * 60);

/// Duration is a delta of time. You can add it to a BlockInfo or Expiration to
/// move that further in the future. Note that an height-based Duration and
/// a time-based Expiration cannot be combined
#[cw_serde]
#[derive(Copy)]
pub enum Duration {
    Height(u64),
    /// Time in seconds
    Time(u64),
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Duration::Height(height) => write!(f, "height: {}", height),
            Duration::Time(time) => write!(f, "time: {}", time),
        }
    }
}

impl Duration {
    /// Create an expiration for Duration after current block
    pub fn after(&self, block: &BlockInfo) -> Expiration {
        match self {
            Duration::Height(h) => Expiration::AtHeight(block.height + h),
            Duration::Time(t) => Expiration::AtTime(block.time.plus_seconds(*t)),
        }
    }

    // creates a number just a little bigger, so we can use it to pass expiration point
    pub fn plus_one(&self) -> Duration {
        match self {
            Duration::Height(h) => Duration::Height(h + 1),
            Duration::Time(t) => Duration::Time(t + 1),
        }
    }
}

impl Add<Duration> for Duration {
    type Output = StdResult<Duration>;

    fn add(self, rhs: Duration) -> StdResult<Duration> {
        match (self, rhs) {
            (Duration::Time(t), Duration::Time(t2)) => Ok(Duration::Time(t + t2)),
            (Duration::Height(h), Duration::Height(h2)) => Ok(Duration::Height(h + h2)),
            _ => Err(StdError::generic_err("Cannot add height and time")),
        }
    }
}

impl Mul<u64> for Duration {
    type Output = Duration;

    fn mul(self, rhs: u64) -> Self::Output {
        match self {
            Duration::Time(t) => Duration::Time(t * rhs),
            Duration::Height(h) => Duration::Height(h * rhs),
        }
    }
}
