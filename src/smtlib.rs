use crate::ast::*;

pub fn parse(input: &str) -> Problem {
    let mut parser = Parser::new(input);
    parser.parse_problem()
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn new<S: Into<String>>(input: S) -> Self {
        Self {
            pos: 0,
            input: input.into(),
        }
    }

    fn parse_problem(&mut self) -> Problem {
        let mut problem = Vec::new();

        loop {
            self.consume_whitespace();
            if self.is_eof() {
                break;
            }

            problem.push(self.parse_statement());
        }

        problem
    }

    fn parse_statement(&mut self) -> Statement {
        assert_eq!(self.consume_char(), '(');

        let stmt = match self.consume_string().as_str() {
            "assert" => Statement::Assert(self.parse_expression()),
            "declare-fun" => {
                self.consume_whitespace();
                let name = self.consume_string();

                self.consume_whitespace();
                assert_eq!(self.consume_char(), '(');
                assert_eq!(self.consume_char(), ')');

                self.consume_whitespace();
                Statement::Declare(name, self.parse_variable_type())
            }
            _ => panic!(),
        };

        self.consume_whitespace();
        assert_eq!(self.consume_char(), ')');

        stmt
    }

    fn parse_variable_type(&mut self) -> VariableType {
        if self.try_consume("Bool") {
            return VariableType::Bool;
        }

        assert!(self.try_consume("(_ BitVec "));

        let len = self.consume_number();
        assert_eq!(self.consume_char(), ')');

        VariableType::BitVector(len)
    }

    fn parse_expression(&mut self) -> Expression {
        self.consume_whitespace();

        match self.peek_char() {
            '#' => {
                return self.parse_constant();
            }
            c if c.is_ascii_alphabetic() => {
                let name = self.consume_string();
                return Expression::Variable(name);
            }
            _ => {}
        }

        assert_eq!(self.consume_char(), '(');

        if self.peek_char() == '=' {
            assert_eq!(self.consume_char(), '=');
            let expr = Expression::Eq(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            );
            assert_eq!(self.consume_char(), ')');
            return expr;
        }

        let op = self.consume_string();
        let expr = match op.as_str() {
            "not" => Expression::Not(Box::new(self.parse_expression())),
            "and" => Expression::And(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "or" => Expression::Or(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "xor" => Expression::Xor(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),

            "bvnot" => Expression::BvNot(Box::new(self.parse_expression())),
            "bvand" => Expression::BvAnd(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvor" => Expression::BvOr(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvxor" => Expression::BvXor(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvadd" => Expression::BvAdd(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvsub" => Expression::BvSub(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvmul" => Expression::BvMul(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvshl" => Expression::BvShl(Box::new(self.parse_expression()), self.consume_number()),
            "bvshr" => Expression::BvShr(Box::new(self.parse_expression()), self.consume_number()),
            "bvult" => Expression::BvUlt(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvule" => Expression::BvUle(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvugt" => Expression::BvUgt(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            "bvuge" => Expression::BvUge(
                Box::new(self.parse_expression()),
                Box::new(self.parse_expression()),
            ),
            _ => panic!(),
        };

        assert_eq!(self.consume_char(), ')');

        expr
    }

    fn parse_constant(&mut self) -> Expression {
        assert_eq!(self.consume_char(), '#');

        match self.consume_char() {
            'b' => {
                let literal = self.consume_while(|c| matches!(c, '0' | '1'));
                let value = usize::from_str_radix(&literal, 2).unwrap();
                Expression::Constant(value, literal.len())
            }
            'x' => {
                let literal = self.consume_while(|c| matches!(c, '0'..='9' | 'a'..='f'));
                let value = usize::from_str_radix(&literal, 16).unwrap();
                Expression::Constant(value, literal.len() * 4)
            }
            _ => panic!(),
        }
    }

    fn consume_number(&mut self) -> usize {
        let int_literal = self.consume_while(|c| c.is_ascii_digit());
        int_literal.parse().unwrap()
    }

    fn consume_string(&mut self) -> String {
        self.consume_while(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(|c| c.is_ascii_whitespace())
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.is_eof() && test(self.peek_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn try_consume(&mut self, s: &str) -> bool {
        if self.starts_with(s) {
            self.pos += s.len();

            return true;
        }

        false
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn peek_char(&mut self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn consume_char(&mut self) -> char {
        let cur_char = self.peek_char();
        self.pos += 1;
        cur_char
    }
}
