/// Conditionally consumes the operator.
///
/// * If the `mapper` returns `true`, then the operator is consumed.
/// * Otherwise, when `false` is returned, then an `Err(NoMatch)` is emitted and the operator is **Not** consumed.
///
/// See also
/// * [`operator_result()`]
/// * [`operator_when()`]
pub(in crate::parser::combinators) fn operator_if(
    pred: impl Fn(OperatorKind) -> bool
)
    -> OperatorCondCombi<
        impl Fn(OperatorKind) -> ConsumerResult<OperatorKind>,
        OperatorKind
    >
{
    operator_result(move |op| Ok(pred(op).then_some(op)))
}
/// Maps the operator before consuming it.
///
/// * If the `mapper` returns `Some(_)`, then the operator is consumed.
/// * Otherwise, when `None` is returned, then an `Err(NoMatch)` is emitted and the operator is **Not** consumed.
///
/// See also
/// * [`operator_result()`]
/// * [`operator_if()`]
pub(in crate::parser::combinators) fn operator_when<O>(
    mapper: impl Fn(OperatorKind) -> Option<O>
)
    -> OperatorCondCombi<
        impl Fn(OperatorKind) -> ConsumerResult<O>,
        O
    >
{
    operator_result(move |op| Ok(mapper(op)))
}

/// Maps the operator before consuming it.
///
/// * If the `mapper` returns `Some(_)`, then the operator is consumed.
/// * Otherwise, when `None` is returned, then an `Err(NoMatch)` is emitted and the operator is **Not** consumed.
///
/// See also
/// * [`operator_result()`]
/// * [`operator_if()`]
pub(in crate::parser::combinators) fn operator_result<O>(
    mapper: impl Fn(OperatorKind) -> ConsumerResult<O>
)
    -> OperatorCondCombi<
        impl Fn(OperatorKind) -> ConsumerResult<O>,
        O
    >
{
    OperatorCondCombi {
        mapper,
        boo: PhantomData,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
impl Combinator for OperatorKind {
    type Output = OperatorKind;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| match tok {
            Operator(op) if op == self => Some(*op),
            _ => None,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Eq, PartialEq)]
pub(in crate::parser::combinators) struct OperatorCondCombi<F, O> {
    mapper: F,
    boo: PhantomData<O>
}

impl<F, O> Combinator for OperatorCondCombi<F, O>
where
    F: Fn(OperatorKind) -> ConsumerResult<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<O> {
        stream.consume(|tok|
            match tok {
                Operator(op) => (self.mapper)(*op),
                _ => Ok(None),
            }
        )
    }
}

impl<F, T> Debug for OperatorCondCombi<F, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OperatorCondCombi")
    }
}

use crate::lexer::OperatorKind;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::ConsumerResult;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::token_value::TokenValue::Operator;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
