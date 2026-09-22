//! Bounded host callbacks, not a JavaScript interpreter.
#[derive(Default)] pub struct JsRuntime { pub instruction_budget:u64 }
#[derive(Debug)] pub enum JsError { Unsupported, BudgetExceeded }
impl JsRuntime { pub fn new(budget:u64)->Self{Self{instruction_budget:budget}} pub fn evaluate(&mut self,_source:&str)->Result<(),JsError>{Err(JsError::Unsupported)} }
