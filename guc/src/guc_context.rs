use postgres_basics::Named;

/// Certain options can only be set at certain times.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum GucContext {

    /// These options cannot be set by the user at all, but only through
    /// internal processes ("server_version" is an example). These are GUC
    /// variables only so they can be shown by SHOW, etc.
    #[default]
    Internal,

    /// These options can only be set when the postmaster starts,
    /// either from the configuration file or the command line.
    Postmaster,

    /// These options can only be set at postmaster startup or by changing
    /// the configuration file and sending the HUP signal to the postmaster
    /// or a backend process. (Notice that the signal receipt will not be
    /// evaluated immediately. The postmaster and the backend check it at a
    /// certain point in their main loop. It's safer to wait than to read a
    /// file asynchronously.)
    Sighup,

    /// Same as `Backend`, but additionally, these can be set
    /// from the startup packet only when the user is a superuser.
    SuBackend,

    /// These options can only be set at postmaster startup,
    ///  from the configuration file, or by client request in the connection
    ///  startup packet (e.g., from libpq's PGOPTIONS variable).
    ///
    /// Furthermore, an already-started backend will ignore changes
    ///  to such an option in the configuration file.  The idea is that these
    ///  options are fixed for a given backend once it's started, but they can
    ///  vary across backends.
    Backend,

    /// These options can be set at postmaster startup, with the SIGHUP
    /// mechanism, or from the startup packet or SQL if you're a superuser.
    Suset,

    /// These options can be set by anyone any time.
    Userset,
}

impl Named for GucContext {
    fn name(&self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Postmaster => "postmaster",
            Self::Sighup => "sighup",
            Self::SuBackend => "superuser-backend",
            Self::Backend => "backend",
            Self::Suset => "superuser",
            Self::Userset => "user",
        }
    }
}
