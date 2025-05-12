pub trait ParseReport: SqlReport + HasLocation {}

impl<T> ParseReport for T
where
    T: SqlReport + HasLocation
{}

use crate::HasLocation;
use postgres_basics::elog::SqlReport;
