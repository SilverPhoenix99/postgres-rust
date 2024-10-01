pub trait ParseReport: SqlReport + HasLocation {}

use crate::error::HasLocation;
use postgres_basics::elog::SqlReport;
