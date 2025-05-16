pub trait HasLocation {
    fn location(&self) -> &Location;
}

use pg_basics::Location;
