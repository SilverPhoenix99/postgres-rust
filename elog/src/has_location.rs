pub trait HasLocation {
    fn location(&self) -> &Location;
}

use postgres_basics::Location;
