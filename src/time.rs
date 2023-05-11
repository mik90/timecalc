use crate::operations::BinaryOperator;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TimeUnit {
    Seconds,
    Microseconds,
    Milliseconds,
    Nanoseconds,
}

impl std::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            TimeUnit::Seconds => "sec",
            TimeUnit::Microseconds => "us",
            TimeUnit::Milliseconds => "ms",
            TimeUnit::Nanoseconds => "ns",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug, PartialEq)]
pub struct TimeValue {
    nanoseconds: i64,
    unit: TimeUnit,
}

impl TimeValue {
    pub fn new(count: i64, unit: TimeUnit) -> TimeValue {
        let nanoseconds = match unit {
            TimeUnit::Seconds => count * 1_000_000_000,
            TimeUnit::Milliseconds => count * 1_000_000,
            TimeUnit::Microseconds => count * 1_000,
            TimeUnit::Nanoseconds => count,
        };
        TimeValue { nanoseconds, unit }
    }

    pub fn unit(&self) -> TimeUnit {
        self.unit
    }

    pub fn count(&self) -> f64 {
        let nanos_float = self.nanoseconds as f64;
        match self.unit {
            TimeUnit::Seconds => nanos_float / 1_000_000_000f64,
            TimeUnit::Milliseconds => nanos_float / 1_000_000f64,
            TimeUnit::Microseconds => nanos_float / 1_000f64,
            TimeUnit::Nanoseconds => nanos_float,
        }
    }

    pub fn modify(&mut self, op: BinaryOperator, rhs: TimeValue) {
        let lhs = self.nanoseconds;
        let rhs = rhs.nanoseconds;
        self.nanoseconds = match op {
            BinaryOperator::Plus => lhs + rhs,
            BinaryOperator::Minus => lhs - rhs,
        };
    }

    pub fn change_unit(&mut self, unit: TimeUnit) {
        self.unit = unit;
    }

    pub fn nanos(&self) -> i64 {
        self.nanoseconds
    }
}

impl std::fmt::Display for TimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.count(), self.unit())
    }
}
