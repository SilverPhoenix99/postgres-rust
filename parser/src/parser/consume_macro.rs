
/// E.g.:
/// * `Double, Precision => ...`
/// * `National, (Character | Char) => ...`
#[deprecated]
macro_rules! consume {

    (
        $self:ident
        Ok {
            $(
                $token:pat $(if $guard:expr)? $(, $($next_token:pat),+)? => $body:expr
            ),+
            $(,)?
        }
        Err { $($err_pattern:pat $(if $err_guard:expr)? => $err_body:expr),+ $(,)? }
    ) => {{
        let buf = &mut $self.buffer;
        match buf.peek() {
            $(Ok($token) $(if $guard)? => {
                buf.next();
                $($(
                    let result = $crate::parser::TokenConsumer::consume(buf, |tok| matches!(tok, $next_token));
                    $crate::parser::result::Required::required(result)?;
                )+)?
                $body
            })+
            $($err_pattern $(if $err_guard)? => Err($err_body)),+
        }
    }};
}

pub(super) use consume;
