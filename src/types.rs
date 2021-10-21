#[derive(Debug)]
pub struct Cnf {
    pub clauses: Vec<Clause>,
}

#[derive(Debug)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug)]
pub struct Literal {
    pub name: String,
    pub inverted: bool,
}

impl Cnf {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Self { clauses }
    }
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }
}

impl Literal {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();
        match name.chars().nth(0).unwrap() {
            '-' => Self {
                name: name[1..].to_string(),
                inverted: true,
            },
            _ => Self {
                name,
                inverted: false,
            },
        }
    }
}
