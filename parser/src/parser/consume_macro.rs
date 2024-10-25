
macro_rules! consume {

    (
        $self:ident
        ok { $($pattern:pat $(if $guard:expr)? => $body:expr),+ $(,)? }
        err { $($err_pattern:pat $(if $err_guard:expr)? => $err_body:expr),+ $(,)? }
    ) => {
        match $self.buffer.peek() {
            $($pattern $(if $guard)? => {
                $self.buffer.next();
                $body
            })+
            $($err_pattern $(if $err_guard)? => $err_body),+
        }
    };
}

pub(super) use consume;
