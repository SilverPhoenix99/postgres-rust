
macro_rules! consume {

    // Simpler form, that only needs to consume 1 token
    (
        $self:ident
        Ok {
            $($token:pat $(if $guard:expr)? => $body:expr),+
            $(,)?
        }
        Err { $($err_pattern:pat $(if $err_guard:expr)? => $err_body:expr),+ $(,)? }
    ) => {
        match $self.buffer.peek() {
            $(Ok($token) $(if $guard)? => {
                $self.buffer.next();
                $body
            })+
            $($err_pattern $(if $err_guard)? => Err($err_body)),+
        }
    };

    // More complex form, that allows consuming a sequence of (required) tokens.
    // E.g.:
    // * Double, Precision => ...
    // * National, (Character | Char) => ...
    (
        $self:ident $fn_name:ident
        Ok {
            $(
                $token:pat $(if $guard:expr)? $(, $($next_token:pat),+)? => $body:expr
            ),+
            $(,)?
        }
        Err { $($err_pattern:pat $(if $err_guard:expr)? => $err_body:expr),+ $(,)? }
    ) => {{
        use $crate::parser::{TokenConsumer, result::Required};
        use postgres_basics::fn_info;

        let buf = &mut $self.buffer;
        match buf.peek() {
            $(Ok($token) $(if $guard)? => {
                buf.next();
                $($(
                    buf.consume(|tok| matches!(tok, $next_token))
                        .required(fn_info!($fn_name))?;
                )+)?
                $body
            })+
            $($err_pattern $(if $err_guard)? => Err($err_body)),+
        }
    }};
}

pub(super) use consume;
