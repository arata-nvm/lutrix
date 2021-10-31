use std::fmt;

pub type Problem = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Assert(Expression),
    Declare(String, VariableType),
}

#[derive(Debug)]
pub enum VariableType {
    Bool,
    BitVector(usize),
}

#[derive(Debug)]
pub enum Expression {
    Constant(usize, usize),
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
    BvAdd(Box<Expression>, Box<Expression>),
    BvSub(Box<Expression>, Box<Expression>),
}

pub fn dump(problem: &Problem) -> String {
    problem
        .iter()
        .map(|s| format!("{}", s))
        .collect::<Vec<String>>()
        .join("\n")
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Statement::*;
        match self {
            Assert(expr) => write!(f, "(assert {})", expr),
            Declare(name, typ) => write!(f, "(declare-fun {} () {})", name, typ),
        }
    }
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::VariableType::*;
        match self {
            Bool => write!(f, "Bool"),
            BitVector(len) => write!(f, "(_ BitVec {})", len),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Expression::*;
        match self {
            Constant(val, _) => write!(f, "#x{:x}", val),
            Variable(name) => write!(f, "{}", name),
            Not(expr) => write!(f, "(not {})", expr),
            And(expr1, expr2) => write!(f, "(and {} {})", expr1, expr2),
            Eq(expr1, expr2) => write!(f, "(= {} {})", expr1, expr2),
            Or(expr1, expr2) => write!(f, "(or {} {})", expr1, expr2),
            Xor(expr1, expr2) => write!(f, "(xor {} {})", expr1, expr2),
            BvNot(expr) => write!(f, "(bvnot {})", expr),
            BvAnd(expr1, expr2) => write!(f, "(bvand {} {})", expr1, expr2),
            BvOr(expr1, expr2) => write!(f, "(bvor {} {})", expr1, expr2),
            BvXor(expr1, expr2) => write!(f, "(bvxor {} {})", expr1, expr2),
            BvAdd(expr1, expr2) => write!(f, "(bvadd {} {})", expr1, expr2),
            BvSub(expr1, expr2) => write!(f, "(bvsub {} {})", expr1, expr2),
        }
    }
}
