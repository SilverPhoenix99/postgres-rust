#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RoleSpecError {
    ReservedRoleSpec(&'static str),
    ForbiddenRoleSpec(&'static str),
}
