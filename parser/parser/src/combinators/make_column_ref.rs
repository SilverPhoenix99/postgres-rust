/// Alias: `makeColumnRef`
pub(in crate::combinators) fn make_column_ref(
    name: Str,
    indirection: Option<Located<Vec<Indirection>>>
)
    -> scan::Result<ColumnRef>
{
    /*
        - splits the name (`.`) from subscripts (`[]`)
        - `*` is only allowed as the last item of the whole source list, if there are no indexes.
        - `*` is allowed in any position after an index, and can show multiple times.
        - examples:
            - `x` -> `(["x"], [])`
            - `x.y` -> `(["x", "y"], [])`
            - `x.y.*` -> `(["x", "y", Wildcard], [])`
            - `x.y[0].*.foo.bar.*` -> `(["x", "y"], [Index(0), Wildcard, "foo", "bar", Wildcard])`
    */

    let Some((indirection, loc)) = indirection else {
        return Ok(SingleName(name))
    };

    let index = indirection.iter()
        .position(|el|
            !matches!(el, Property(_))
        );

    let (qname, indirection) = if let Some(index) = index {

        let mut qname = indirection;
        let indirection = qname.split_off(index);

        (qname, indirection)
    }
    else {
        // Easy case: they're all properties
        (indirection, vec![])
    };

    let mut qname: Vec<_> = qname.into_iter()
        .map(|el| {
            if let Property(name) = el {
                name
            } else {
                unreachable!("This should never happen, as we already checked the index")
            }
        })
        .collect();

    qname.insert(0, name);

    match indirection.as_slice() {
        // `Wildcard` is not the last element
        [Wildcard, _, ..] => Err(ImproperUseOfStar.at(loc).into()),
        [Wildcard] => Ok(WildcardName(qname)),
        [] => Ok(Name(qname)),
        // `Wildcard` and `Property` won't be the 1st element,
        // so the indirection starts with `Index` or `Slice`
        _ => Ok(IndirectionRef { name: qname, indirection }),
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_basics::Location;
    use test_case::test_case;

    #[test_case("name", None, SingleName("name".into()))]
    #[test_case("name",
        Some((
            vec![Wildcard],
            Location::new(0..0, 0, 0)
        )),
        WildcardName(vec!["name".into()])
    )]
    #[test_case(
        "name",
        Some((
            vec![Property("foo".into())],
            Location::new(0..0, 0, 0)
        )),
        Name(vec!["name".into(), "foo".into()])
    )]
    #[test_case(
        "name",
        Some((
            vec![Property("foo".into()), Wildcard],
            Location::new(0..0, 0, 0)
        )),
        WildcardName(vec!["name".into(), "foo".into()])
    )]
    fn test_make_column_ref(
        name: &'static str,
        indirection: Option<Located<Vec<Indirection>>>,
        expected: ColumnRef
    )
        -> scan::Result<()>
    {
        let actual = make_column_ref(name.into(), indirection)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}

use crate::scan;
use pg_ast::ColumnRef;
use pg_ast::ColumnRef::Indirection as IndirectionRef;
use pg_ast::ColumnRef::Name;
use pg_ast::ColumnRef::SingleName;
use pg_ast::ColumnRef::WildcardName;
use pg_ast::Indirection;
use pg_ast::Indirection::Property;
use pg_ast::Indirection::Wildcard;
use pg_basics::Located;
use pg_basics::Str;
use pg_elog::parser::Error::ImproperUseOfStar;
