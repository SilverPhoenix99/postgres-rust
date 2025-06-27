pub(super) fn strip_delimiters(kind: StringKind, slice: &str) -> &str {

    let range = match kind {
        StringKind::Dollar => {
            let delim_len = slice.chars()
                .enumerate()
                .skip(1)
                .find(|(_, c)| *c == '$')
                .map(|(i, _)| i + 1) // include the '$'
                .expect("$-string delimiter should exist");

            let str_end = slice.len() - delim_len;
            delim_len..str_end
        }
        StringKind::Basic { .. } => 1..(slice.len() - 1),
        StringKind::Extended { .. } => {
            // `e'`, `n'`, or `'`
            let delim_len = if slice.starts_with('\'') { 1 } else { 2 };
            delim_len..(slice.len() - 1)
        }
        StringKind::Unicode => 3..(slice.len() - 1),
    };

    &slice[range]
}

use pg_lexer::StringKind;
