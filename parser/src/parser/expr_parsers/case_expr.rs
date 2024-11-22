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
            .maybe_match()?;

        let target = self.a_expr().try_match()?;

        let mut when_clauses = vec![];

        while keyword(When).parse(&mut self.buffer).try_match()?.is_some() {

            let condition = self.a_expr().required()?;
            keyword(Then).required().parse(&mut self.buffer)?;
            let body = self.a_expr().required()?;

            let clause = CaseWhen::new(condition, body);
            when_clauses.push(clause);
        }

        if when_clauses.is_empty() {
            let loc = self.buffer.current_location();
            return Err(syntax_err(loc).into());
        }

        let default = if keyword(Else).parse(&mut self.buffer).try_match()?.is_some() {
            Some(self.a_expr().required()?)
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

use crate::lexer::Keyword::Case;
use crate::lexer::Keyword::Else;
use crate::lexer::Keyword::Then;
use crate::lexer::Keyword::When;
use crate::parser::ast_node::CaseExpr;
use crate::parser::ast_node::CaseWhen;
use crate::parser::combinators::keyword;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::error::syntax_err;
use crate::parser::result::Required;
use crate::parser::result::ScanResult;
use crate::parser::result::ScanResultTrait;
use crate::parser::result::TryMatch;
use crate::parser::Parser;
