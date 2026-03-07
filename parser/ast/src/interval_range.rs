#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntervalRange {
    Full { precision: Option<i32> },
    Year,
    YearToMonth,
    Month,
    Day,
    DayToHour,
    DayToMinute,
    DayToSecond { precision: Option<i32> },
    Hour,
    HourToMinute,
    HourToSecond { precision: Option<i32> },
    Minute,
    MinuteToSecond { precision: Option<i32> },
    Second { precision: Option<i32> },
}

impl Default for IntervalRange {
    fn default() -> Self {
        Self::Full { precision: None }
    }
}
