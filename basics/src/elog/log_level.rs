
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LogLevel {
    /// Debugging messages, in categories of decreasing detail.
    Debug5            = 10,
    /// Debugging messages, in categories of decreasing detail.
    Debug4            = 11,
    /// Debugging messages, in categories of decreasing detail.
    Debug3            = 12,
    /// Debugging messages, in categories of decreasing detail.
    Debug2            = 13,
    /// used by GUC `debug_*` variables
    Debug1            = 14,
    /// Server operational messages; sent only to server log by default.
    Log               = 15,
    /// Same as LOG for server reporting, but never sent to client.
    /// Alias: `CommError`
    LogServerOnly     = 16,
    /// Messages specifically requested by user (eg `VACUUM VERBOSE` output); always sent to
    /// client regardless of `client_min_messages`, but by default not sent to server log.
    Info              = 17,
    /// Helpful messages to users about query operation;
    /// sent to client and not to server log by default.
    ///
    /// `Notice` is for expected messages like implicit sequence creation by `SERIAL`.
    Notice            = 18,
    /// `Warning` is for unexpected messages.
    Warning           = 19,
    /// Warnings to be sent to client as usual, but never to the server log.
    WarningClientOnly = 20,
    /// User error - abort transaction; return to known state
    Error             = 21,
    /// Fatal error - abort process
    Fatal             = 22,
    /// Take down the other backends with me
    Panic             = 23,
}
