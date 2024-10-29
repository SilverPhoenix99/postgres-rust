
macro_rules! consume {

    (
        $self:ident
        Ok { $($pattern:pat $(if $guard:expr)? => $body:expr),+ $(,)? }
        Err { $($err_pattern:pat $(if $err_guard:expr)? => $err_body:expr),+ $(,)? }
    ) => {
        match $self.buffer.peek() {
            $(Ok($pattern) $(if $guard)? => {
                $self.buffer.next();
                $body
            })+
            $($err_pattern $(if $err_guard)? => Err($err_body)),+
        }
    };
}

pub(super) use consume;
