//! CSS data contracts. No external stylesheet parser is used.
#[derive(Clone,Debug,Default)] pub struct Stylesheet { pub rules:Vec<Rule> }
#[derive(Clone,Debug)] pub struct Rule { pub selector:String, pub declarations:Vec<Declaration> }
#[derive(Clone,Debug)] pub struct Declaration { pub property:String, pub value:String }
