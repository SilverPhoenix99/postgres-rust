impl Parser<'_> {

    pub(super) fn privileges(&mut self) -> ScanResult<AccessPrivilege> {

        /*
              ALL ( PRIVILEGES )? opt_column_list
            | privilege_list
        */

        if self.buffer.consume_kw_eq(All).no_match_to_option()?.is_some() {
            self.buffer.consume_kw_eq(Privileges).optional()?;
            let columns = self.opt_column_list().optional()?;
            return Ok(AccessPrivilege::All(columns))
        }

        let privileges = self.privilege_list()?;
        Ok(AccessPrivilege::Specific(privileges))
    }

    /// Post-condition: Vec is **Not** empty
    pub(super) fn privilege_list(&mut self) -> ScanResult<Vec<SpecificAccessPrivilege>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::privilege_list";

        /*
            privilege ( ',' privilege )*
        */

        let element = self.privilege()?;
        let mut elements = vec![element];

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let element = self.privilege().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    fn privilege(&mut self) -> ScanResult<SpecificAccessPrivilege> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::privilege";

        /*
              ALTER SYSTEM
            | SELECT opt_column_list
            | REFERENCES opt_column_list
            | CREATE opt_column_list
            | col_id opt_column_list
        */

        let privilege = consume!{self
            Ok {
                Kw(Alter) => {
                    self.buffer.consume_kw_eq(SystemKw).required(fn_info!(FN_NAME))?;
                    Ok(AlterSystem)
                },
                Kw(CreateKw) => {
                    let columns = self.opt_column_list().optional()?;
                    Ok(Create(columns))
                },
                Kw(ReferencesKw) => {
                    let columns = self.opt_column_list().optional()?;
                    Ok(References(columns))
                },
                Kw(SelectKw) => {
                    let columns = self.opt_column_list().optional()?;
                    Ok(Select(columns))
                },
            }
            Err {
                Ok(_) => NoMatch,
                Err(err) => err.into(),
            }
        };

        if let Some(privilege) = privilege.no_match_to_option()? {
            return Ok(privilege);
        }

        let name = self.col_id()?;
        let columns = self.opt_column_list().optional()?;

        Ok(Named(name, columns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("all")]
    #[test_case("all privileges")]
    fn test_all_privileges(source: &str) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(AccessPrivilege::All(None)), parser.privileges());
    }

    #[test]
    fn test_all_privileges_with_columns() {
        let source = "all (column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let expected = vec!["column_name".into()];
        assert_eq!(Ok(AccessPrivilege::All(Some(expected))), parser.privileges());
    }

    #[test]
    fn test_specific_privileges() {
        let source = "select, references";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let expected = vec![
            Select(None),
            References(None),
        ];
        assert_eq!(Ok(AccessPrivilege::Specific(expected)), parser.privileges());
    }

    #[test]
    fn test_name_privilege() {
        let source = "some_name another_name(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(Named("some_name".into(), expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(Named("another_name".into(), expected)), parser.privilege());
    }

    #[test]
    fn test_select_privilege() {
        let source = "select select(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(Select(expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(Select(expected)), parser.privilege());
    }

    #[test]
    fn test_references_privilege() {
        let source = "references references(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(References(expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(References(expected)), parser.privilege());
    }

    #[test]
    fn test_create_privilege() {
        let source = "create create(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(Create(expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(Create(expected)), parser.privilege());
    }

    #[test]
    fn test_alter_system_privilege() {
        let source = "alter system";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(AlterSystem), parser.privilege());
    }

    #[test]
    fn test_privilege_list() {
        let source = "alter system, select, create, some_privilege";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = vec![
            AlterSystem,
            Select(None),
            Create(None),
            Named("some_privilege".into(), None),
        ];

        assert_eq!(Ok(expected), parser.privilege_list());
    }
}

use crate::{
    lexer::{
        Keyword::{
            All,
            Alter,
            Create as CreateKw,
            Privileges,
            References as ReferencesKw,
            Select as SelectKw,
            SystemKw
        },
        TokenKind::{Comma, Keyword as Kw}
    },
    parser::{
        ast_node::{
            AccessPrivilege,
            SpecificAccessPrivilege::{self, *},
        },
        consume_macro::consume,
        result::{
            Optional,
            Required,
            ScanErrorKind::NoMatch,
            ScanResult,
            ScanResultTrait
        },
        Parser
    }
};
use postgres_basics::fn_info;
