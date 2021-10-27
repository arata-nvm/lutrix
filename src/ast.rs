#[derive(Debug)]
pub enum Node {
    Assert(Box<Node>),

    Variable(String),
}
