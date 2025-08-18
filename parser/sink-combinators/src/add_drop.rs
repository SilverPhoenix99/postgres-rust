pub fn add_drop(ctx: &mut ParserContext) -> scan::Result<AddDrop> {

    /*
          ADD
        | DROP
    */

    let kw = alt!(Kw::Add, DropKw).parse(ctx)?;

    let op = if kw == Kw::Add { Add } else { Drop };

    Ok(op)
}

use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::DropKw;
use pg_parser_core::scan;
use pg_sink_ast::AddDrop;
use pg_sink_ast::AddDrop::Add;
use pg_sink_ast::AddDrop::Drop;
