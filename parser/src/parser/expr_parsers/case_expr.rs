impl Parser<'_> {
    pub(in crate::parser) fn case_expr(&mut self) -> ScanResult<CaseExpr> {

        /*
            CASE ( a_expr )?
                ( WHEN a_expr THEN a_expr )+
                ( ELSE a_expr )?
            END
        */

        keyword(Case)
            .parse(&mut self.buffer)
            .no_match_to_option()?;

        let target = self.a_expr().try_match(fn_info!())?;

        let mut when_clauses = vec![];

        while keyword(When).parse(&mut self.buffer).try_match(fn_info!())?.is_some() {

            let condition = self.a_expr().required(fn_info!())?;
            keyword(Then).required(fn_info!()).parse(&mut self.buffer)?;
            let body = self.a_expr().required(fn_info!())?;

            let clause = CaseWhen::new(condition, body);
            when_clauses.push(clause);
        }

        if when_clauses.is_empty() {
            let loc = self.buffer.current_location();
            return Err(syntax_err(fn_info!(), loc).into());
        }

        let default = if keyword(Else).parse(&mut self.buffer).try_match(fn_info!())?.is_some() {
            Some(self.a_expr().required(fn_info!())?)
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
    lexer::Keyword::{Case, Else, Then, When},
    parser::{
        ast_node::{CaseExpr, CaseWhen},
        combinators::{keyword, ParserFunc, ParserFuncHelpers},
        error::syntax_err,
        result::{Required, ScanResult, ScanResultTrait, TryMatch},
        Parser
    }
};
use postgres_basics::fn_info;
