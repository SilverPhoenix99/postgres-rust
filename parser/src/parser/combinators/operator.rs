pub(in crate::parser) fn operator(operator: OperatorKind) -> OperatorCombi {
    OperatorCombi(operator)
}

/// Conditionally consumes the operator.
///
/// * If the `mapper` returns `true`, then the operator is consumed.
/// * Otherwise, when `false` is returned, then an `Err(NoMatch)` is emitted and the operator is **Not** consumed.
///
/// See also
/// * [`operator_result()`]
/// * [`operator_when()`]
pub(in crate::parser) fn operator_if(
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
pub(in crate::parser) fn operator_when<F, O>(
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
pub(in crate::parser) fn operator_result<O>(
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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct OperatorCombi(OperatorKind);

impl Combinator for OperatorCombi {
    type Output = OperatorKind;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok|
            tok.operator().filter(|op| *op == self.0)
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Eq, PartialEq)]
pub(in crate::parser) struct OperatorCondCombi<F, O> {
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
            match tok.operator() {
                Some(op) => (self.mapper)(op),
                None => Ok(None),
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
use crate::parser::combinators::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::{ConsumerResult, TokenConsumer, TokenStream};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
