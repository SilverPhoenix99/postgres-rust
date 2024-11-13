/// Returns UESCAPE's escape char if the string is valid.
#[inline] // Only called from a single place
pub(super) fn uescape_escape(source: &str) -> Option<char> {

    if source.len() != 3 {
        // Only (some) ASCII chars are acceptable as the escape char
        return None
    }

    let mut chars = source.chars();
    if !chars.next().is_some_and(|c| c == '\'') {
        return None
    }

    let escape = chars.next()?;
    if is_hex_digit(escape)
        || is_whitespace(escape)
        || escape == '+'
        || escape == '\''
        || escape == '"'
    {
        return None
    }

    match chars.next() {
        Some('\'') => Some(escape),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uescape_escape() {
        assert_eq!(uescape_escape(""), None);
        assert_eq!(uescape_escape("''"), None);
        assert_eq!(uescape_escape("'' "), None);
        assert_eq!(uescape_escape("' '"), None);
        assert_eq!(uescape_escape("'a'"), None);
        assert_eq!(uescape_escape("'f'"), None);
        assert_eq!(uescape_escape("'0'"), None);
        assert_eq!(uescape_escape("'9'"), None);
        assert_eq!(uescape_escape("'+'"), None);
        assert_eq!(uescape_escape("'''"), None);
        assert_eq!(uescape_escape(r#"'"'"#), None);
        assert_eq!(uescape_escape("'-'"), Some('-'));
        assert_eq!(uescape_escape("'z'"), Some('z'));
        assert_eq!(uescape_escape("'!'"), Some('!'));
    }
}

use postgres_basics::ascii::{is_hex_digit, is_whitespace};
