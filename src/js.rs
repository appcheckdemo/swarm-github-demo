//! A deliberately small, bounded scripting facade.  It never embeds an engine.
use std::collections::HashMap;

#[derive(Clone,Debug,PartialEq)] pub struct Script { pub source:String }
#[derive(Debug,Clone,PartialEq)] pub enum JsError { Syntax(String), Runtime(String), BudgetExceeded }
#[derive(Clone,Debug,PartialEq)] pub enum Value { Undefined, Null, Bool(bool), Number(f64), String(String), Array(Vec<Value>), Object(HashMap<String,Value>) }
impl Default for Value {fn default()->Self{Self::Undefined}}
impl Value { fn num(&self)->f64{match self{Self::Number(n)=>*n,Self::Bool(b)=>if *b{1.}else{0.},Self::String(s)=>s.parse().unwrap_or(0.),_=>0.}} fn truth(&self)->bool{match self{Self::Undefined|Self::Null=>false,Self::Bool(b)=>*b,Self::Number(n)=>*n!=0.,Self::String(s)=>!s.is_empty(),_=>true}} }

pub trait Host { fn call(&mut self, _name:&str, _args:&[Value])->Result<Value,JsError>{Ok(Value::Undefined)} }
#[derive(Default)] pub struct NullHost; impl Host for NullHost {}
pub struct Runtime { scripts:Vec<Script>, vars:HashMap<String,Value>, pub instruction_budget:u64 }
impl Default for Runtime {fn default()->Self{Self::new()}}
impl Runtime {
 pub fn new()->Self{Self{scripts:vec![],vars:HashMap::new(),instruction_budget:100_000}}
 pub fn with_budget(b:u64)->Self{Self{instruction_budget:b,..Self::new()}}
 pub fn enqueue(&mut self,s:impl Into<String>){self.scripts.push(Script{source:s.into()})}
 pub fn pending(&self)->&[Script]{&self.scripts} pub fn drain(&mut self)->Vec<Script>{std::mem::take(&mut self.scripts)} pub fn is_empty(&self)->bool{self.scripts.is_empty()}
 pub fn globals(&self)->&HashMap<String,Value>{&self.vars}
 pub fn evaluate(&mut self,source:&str)->Result<Value,JsError>{let mut h=NullHost;self.evaluate_with(source,&mut h)}
 pub fn evaluate_with<H:Host>(&mut self,source:&str,host:&mut H)->Result<Value,JsError>{let mut budget=self.instruction_budget;let mut last=Value::Undefined;for statement in source.split(';'){let s=statement.trim();if s.is_empty(){continue} if budget==0{return Err(JsError::BudgetExceeded)} budget-=1;last=self.statement(s,host)?;}Ok(last)}
 fn statement<H:Host>(&mut self,s:&str,h:&mut H)->Result<Value,JsError>{let s=s.trim(); if s.starts_with("throw"){return Err(JsError::Runtime(s[5..].trim().to_string()))} if let Some((n,e))=s.split_once('='){let n=n.trim().trim_start_matches("let ").trim_start_matches("var ");if n.chars().all(|c|c.is_alphanumeric()||c=='_'){let v=self.expr(e.trim(),h)?;self.vars.insert(n.into(),v.clone());return Ok(v)}} self.expr(s,h)}
 fn expr<H:Host>(&mut self,s:&str,h:&mut H)->Result<Value,JsError>{let s=s.trim();for op in ["||","&&","==","!=","<=",">=","+","-","*","/","<",">"]{if let Some(i)=s.find(op){let a=self.expr(&s[..i],h)?;let b=self.expr(&s[i+op.len()..],h)?;return Ok(match op{ "+"=>Value::Number(a.num()+b.num()),"-"=>Value::Number(a.num()-b.num()),"*"=>Value::Number(a.num()*b.num()),"/"=>Value::Number(a.num()/b.num()),"=="=>Value::Bool(a==b),"!="=>Value::Bool(a!=b),"<"=>Value::Bool(a.num()<b.num()),">"=>Value::Bool(a.num()>b.num()),"&&"=>Value::Bool(a.truth()&&b.truth()),"||"=>Value::Bool(a.truth()||b.truth()),_=>Value::Undefined})}} if s.starts_with('"')&&s.ends_with('"'){return Ok(Value::String(s[1..s.len()-1].into()))} if let Ok(n)=s.parse(){return Ok(Value::Number(n))} if s=="true"{return Ok(Value::Bool(true))} if s=="false"{return Ok(Value::Bool(false))} if let Some(v)=self.vars.get(s){return Ok(v.clone())} if let Some(p)=s.find('('){if s.ends_with(')'){let name=s[..p].trim();return h.call(name,&[])}} Err(JsError::Syntax(s.into()))}
}
pub type JsRuntime = Runtime;
#[cfg(test)] mod tests {use super::*;#[test]fn arithmetic(){let mut r=Runtime::new();assert_eq!(r.evaluate("x=2+3").unwrap(),Value::Number(5.));}#[test]fn budget(){let mut r=Runtime::with_budget(1);assert_eq!(r.evaluate("a=1;b=2"),Err(JsError::BudgetExceeded));}}
