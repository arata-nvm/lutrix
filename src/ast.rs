pub type Problem = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Assert(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Variable(String),
    Not(Box<Expression>),

    And(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
}
