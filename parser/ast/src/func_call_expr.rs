#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncCallExpr {
    func: FuncCall,
    agg_filter: Option<ExprNode>,
    over: Option<OverClause>,
}

impl FuncCallExpr {
    pub fn new(func: FuncCall) -> Self {
        Self {
            func,
            agg_filter: None,
            over: None
        }
    }

    pub fn func(&self) -> &FuncCall {
        &self.func
    }

    pub fn set_agg_filter(&mut self, agg_filter: Option<ExprNode>) -> &mut Self {
        self.agg_filter = agg_filter;
        self
    }
    
    pub fn with_agg_filter(mut self, agg_filter: ExprNode) -> Self {
        self.agg_filter = Some(agg_filter);
        self
    }
    
    pub fn agg_filter(&self) -> Option<&ExprNode> {
        self.agg_filter.as_ref()
    }

    pub  fn set_over(&mut self, over: Option<OverClause>) -> &mut Self {
        self.over = over;
        self
    }
    
    pub fn with_over(mut self, over: OverClause) -> Self {
        self.over = Some(over);
        self
    }

    pub fn over(&self) -> Option<&OverClause> {
        self.over.as_ref()
    }
}

impl From<FuncCall> for FuncCallExpr {
    fn from(func: FuncCall) -> Self {
        Self::new(func)
    }
}

use crate::ExprNode;
use crate::FuncCall;
use crate::OverClause;
