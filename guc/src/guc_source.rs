/**
 * The following type records the source of the current setting.  A
 * new setting can only take effect if the previous setting had the
 * same or lower level.  (E.g, changing the config file doesn't
 * override the postmaster command line.)  Tracking the source allows us
 * to process sources in any convenient order without affecting results.
 * Sources <= `OVERRIDE` will set the default used by RESET, as well
 * as the current value.
 *
 * `INTERACTIVE` isn't actually a source value, but is the
 * dividing line between "interactive" and "non-interactive" sources for
 * error reporting purposes.
 *
 * `TEST` is used when testing values to be used later.  For example,
 * ALTER DATABASE/ROLE tests proposed per-database or per-user defaults this
 * way, and CREATE FUNCTION tests proposed function SET clauses this way.
 * This is an interactive case, but it needs its own source value because
 * some assign hooks need to make different validity checks in this case.
 * In particular, references to nonexistent database objects generally
 * shouldn't throw hard errors in this case, at most NOTICEs, since the
 * objects might exist by the time the setting is used for real.
 *
 * When setting the value of a non-compile-time-constant PGC_INTERNAL option,
 * source == `DYNAMIC_DEFAULT` should typically be used so that the value
 * will show as "default" in pg_settings.  If there is a specific reason not
 * to want that, use source == `OVERRIDE`.
 *
 * NB: see GucSource_Names in guc.c if you change this.
 */
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum GucSource {
    #[default]
    Default,        /* hard-wired default ("boot_val") */
    DynamicDefault, /* default computed during initialization */
    EnvVar,         /* postmaster environment variable */
    File,           /* postgresql.conf */
    Argv,           /* postmaster command line */
    Global,         /* global in-database setting */
    Database,       /* per-database setting */
    User,           /* per-user setting */
    DatabaseUser,   /* per-user-and-database setting */
    Client,         /* from client connection request */
    Override,       /* special case to forcibly set default */
    Interactive,    /* dividing line for error reporting */
    Test,           /* test per-database or per-user setting */
    Session,        /* SET command */
}

impl Named for GucSource {
    /**
     * Displayable names for source types (enum GucSource)
     *
     * Note: these strings are deliberately not localized.
     */
    fn name(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::DynamicDefault => "default",
            Self::EnvVar => "environment variable",
            Self::File => "configuration file",
            Self::Argv => "command line",
            Self::Global => "global",
            Self::Database => "database",
            Self::User => "user",
            Self::DatabaseUser => "database user",
            Self::Client => "client",
            Self::Override => "override",
            Self::Interactive => "interactive",
            Self::Test => "test",
            Self::Session => "session",
        }
    }
}

use postgres_basics::Named;
