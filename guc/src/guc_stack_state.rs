/**
 * Stack entry for saving the state a variable had prior to an uncommitted
 * transactional change.
 *
 * This is almost `GucAction`, but we need a fourth state for SET+LOCAL
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GucStackState {
    /// entry caused by function SET option
    Save,
    /// entry caused by plain SET command
    Set,
    /// entry caused by SET LOCAL command
    Local,
    /// entry caused by SET then SET LOCAL
    SetLocal,
}
