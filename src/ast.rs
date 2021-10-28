#[derive(Debug)]
pub enum Node {
    Assert(Box<Node>),

    Variable(String),
    Not(Box<Node>),

    And(Box<Node>, Box<Node>),
    Eq(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
}
