pub(super) fn add_drop(ctx: &mut ParserContext) -> scan::Result<AddDrop> {

    /*
          ADD
        | DROP
    */

    let kw = alt!(Kw::Add, DropKw).parse(ctx)?;

    let op = if kw == Kw::Add { Add } else { Drop };

    Ok(op)
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::ParserContext;
use pg_ast::AddDrop;
use pg_ast::AddDrop::Add;
use pg_ast::AddDrop::Drop;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::DropKw;
use pg_parser_core::scan;
