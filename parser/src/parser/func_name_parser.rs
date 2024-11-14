impl Parser<'_> {
    pub(in crate::parser) fn func_name(&mut self) -> ScanResult<QualifiedName> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::func_name";

        /*
              type_func_name_keyword
            | col_name_keyword attrs
            | unreserved_keyword ( attrs )?
            | IDENT ( attrs )?
        */

        let kw = self.buffer.consume_kw(|kw|
            matches!(kw.details().category(), Unreserved | ColumnName | TypeFuncName)
        ).no_match_to_option()?;

        let (ident, required_indirection): (CowStr, bool) = if let Some(kw) = kw {
            let name = kw.details().text().into();
            match kw.details().category() {
                TypeFuncName => return Ok(vec![name]),
                ColumnName => (name, true),
                Unreserved => (name, false),
                Reserved => unreachable!("it shouldn't accept Reserved keywords")
            }
        }
        else {
            (self.identifier()?.into(), false)
        };

        let loc = self.buffer.current_location();
        let name = self.attrs(ident)?;

        if required_indirection && name.len() == 1 {
            return Err(syntax_err(fn_info!(FN_NAME), loc).into());
        }

        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_type_func_name_keyword() {
        let source = "authorization";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["authorization".into()]), parser.func_name());
    }

    #[test]
    fn test_col_name_keyword() {
    let source = "trim.something";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["trim".into(), "something".into()]), parser.func_name());
    }

    #[test]
    fn test_unreserved_keyword() {
        let source = "attribute inline.some_thing";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["attribute".into()]), parser.func_name());
        assert_eq!(Ok(vec!["inline".into(), "some_thing".into()]), parser.func_name());
    }

    #[test]
    fn test_identifier() {
        let source = "some_ident another_ident.something";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["some_ident".into()]), parser.func_name());
        assert_eq!(Ok(vec!["another_ident".into(), "something".into()]), parser.func_name());
    }
}

use crate::parser::error::syntax_err;
use crate::parser::QualifiedName;
use crate::{
    lexer::KeywordCategory::{ColumnName, Reserved, TypeFuncName, Unreserved},
    parser::{
        result::{ScanResult, ScanResultTrait},
        CowStr,
        Parser,
    }
};
use postgres_basics::fn_info;
