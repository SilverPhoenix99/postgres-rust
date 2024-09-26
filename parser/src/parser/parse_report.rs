pub trait ParseReport: SqlReport {

    fn location(&self) -> &Location;
}

use postgres_basics::{Location, SqlReport};
