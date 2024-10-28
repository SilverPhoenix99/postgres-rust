impl Parser<'_> {
    pub(in crate::parser) fn case_expr(&mut self) -> ScanResult<CaseExpr> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::case_expr";

        /*
            CASE ( a_expr )?
                ( WHEN a_expr THEN a_expr )+
                ( ELSE a_expr )?
            END
        */

        self.buffer.consume_kw_eq(Keyword::Case).no_match_to_option()?;

        let target = self.a_expr().try_match(fn_info!(FN_NAME))?;

        let mut when_clauses = vec![];

        while self.buffer.consume_kw_eq(When).try_match(fn_info!(FN_NAME))?.is_some() {

            let condition = self.a_expr().required(fn_info!(FN_NAME))?;
            self.buffer.consume_kw_eq(Then).required(fn_info!(FN_NAME))?;
            let body = self.a_expr().required(fn_info!(FN_NAME))?;

            let clause = CaseWhen::new(condition, body);
            when_clauses.push(clause);
        }

        if when_clauses.is_empty() {
            return Err(syntax_err!(FN_NAME))
        }

        let default = if self.buffer.consume_kw_eq(Else).try_match(fn_info!(FN_NAME))?.is_some() {
            Some(self.a_expr().required(fn_info!(FN_NAME))?)
        }
        else {
            None
        };

        Ok(CaseExpr::new(target, when_clauses, default))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_when() {
        // TODO
    }
}

use crate::{
    lexer::Keyword::{self, Else, Then, When},
    parser::{
        ast_node::{CaseExpr, CaseWhen},
        error::syntax_err,
        result::{Required, ScanResult, ScanResultTrait, TryMatch},
        Parser
    }
};
use postgres_basics::fn_info;
