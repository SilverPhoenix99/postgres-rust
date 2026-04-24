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
    use test_case::test_matrix;

    #[test_matrix("" => None ; "empty string")]
    #[test_matrix(" " => None ; "space")]
    #[test_matrix("a" => None)]
    #[test_matrix("f" => None)]
    #[test_matrix("0" => None)]
    #[test_matrix("9" => None)]
    #[test_matrix("+" => None ; "plus sign")]
    #[test_matrix("'" => None ; "single quote")]
    #[test_matrix(r#"""# => None ; "double quote")]
    #[test_matrix("-" => Some('-') ; "minus sign")]
    #[test_matrix("z" => Some('z'))]
    #[test_matrix("!" => Some('!') ; "exclamation mark")]
    fn test_uescape_escape(source: &str) -> Option<char> {
        uescape_escape(source)
    }
}

use pg_basics::ascii::is_hex_digit;
use pg_basics::ascii::is_whitespace;
