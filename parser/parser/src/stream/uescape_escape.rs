/// Returns UESCAPE's escape char if the string is valid.
#[inline] // Only called from a single place
pub(super) fn uescape_escape(source: &str) -> Option<char> {

    if source.len() != 1 {
        // Only (some) ASCII chars are acceptable as the escape char
        return None
    }

    let escape = source.chars().next()?;

    if is_hex_digit(escape)
        || is_whitespace(escape)
        || escape == '+'
        || escape == '\''
        || escape == '"'
    {
        return None
    }

    Some(escape)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test_case("", None ; "empty string")]
    #[test_case(" ", None ; "space")]
    #[test_case("a", None)]
    #[test_case("f", None)]
    #[test_case("0", None)]
    #[test_case("9", None)]
    #[test_case("+", None ; "plus sign")]
    #[test_case("'", None ; "single quote")]
    #[test_case(r#"""#, None ; "double quote")]
    #[test_case("-", Some('-') ; "minus sign")]
    #[test_case("z", Some('z'))]
    #[test_case("!", Some('!') ; "exclamation mark")]
    fn test_uescape_escape(source: &str, expected: Option<char>) {
        assert_eq!(expected, uescape_escape(source));
    }

    use test_case::test_case;
}

use pg_basics::ascii::is_hex_digit;
use pg_basics::ascii::is_whitespace;
