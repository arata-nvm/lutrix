pub type Problem = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Assert(Expression),
    Define(String, VariableType),
}

#[derive(Debug)]
pub enum VariableType {
    Bool,
    BitVector(usize),
}

#[derive(Debug)]
pub enum Expression {
    Variable(String),

    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),

    BvNot(Box<Expression>),
    BvAnd(Box<Expression>, Box<Expression>),
    BvOr(Box<Expression>, Box<Expression>),
    BvXor(Box<Expression>, Box<Expression>),
}
