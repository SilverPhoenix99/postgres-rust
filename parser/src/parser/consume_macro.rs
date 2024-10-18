
macro_rules! consume {

    (
        $self:ident
        default => $default:expr,
        $($pattern:pat $(if $guard:expr)? => $body:expr),+
        $(,)?
    ) => {
        match $self.buffer.peek() {
            Ok((tok, _)) => match tok {
                $(
                    $pattern $(if $guard)? => {
                        $self.buffer.next();
                        $body
                    }
                )+
                _ => $default,
            },
            Err(err) => Err(err.clone().into()),
        }
    };

    (
        $self:ident
        default,
        $($pattern:pat $(if $guard:expr)? => $body:expr),+
        $(,)?
    ) => {
        consume!{$self
            default => Err(Default::default()),
            $($pattern $(if $guard)? => $body),+
        }
    };

    (
        $self:ident
        $($pattern:pat $(if $guard:expr)? => $body:expr),+
        $(,)?
    ) => {
        consume!{$self
            default => Err(ScanErrorKind::NoMatch.into()),
            $($pattern $(if $guard)? => $body),+
        }
    };
}

pub(super) use consume;
