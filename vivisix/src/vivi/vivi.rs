// use crate::vivi::expr;
use std::collections::HashMap;
// use std::error::Error;

trait Expr {
    fn print(&self);
}

struct BinaryExpr {
    lhs: Box<dyn Expr>,
    rhs: Box<dyn Expr>,
    operator: String,
}

struct LiteralExpr {
    num: Option<f32>,
    var: Option<String>,
}

impl LiteralExpr {
    fn new(num: Option<f32>, literal: Option<String>) -> Result<Self, String> {
        if let Some(n) = num {
            return Ok(Self { num: Some(n), var: None });
        } else if let Some(lit) = literal {
            return Ok(Self {num: None, var: Some(lit)});
        } else {
            return Err("Missing params".to_string());
        }
    }
}

impl Expr for LiteralExpr {
    fn print(&self) {
        match self {
            LiteralExpr {num: Some(n), var: None} => print!("{} ", self.num.as_ref().unwrap()),
            LiteralExpr {num: None, var: Some(v)} => print!("{} ", self.var.as_ref().unwrap()),
            _ => println!("")
        }
    }
}

impl BinaryExpr {
    fn new(operator: String, lhs: Box<dyn Expr>, rhs: Box<dyn Expr>) -> Self {
        Self { operator: operator, lhs: lhs, rhs: rhs }
    }
}

impl Expr for BinaryExpr {
    fn print(&self) {
        self.lhs.print();
        print!("{} ", self.operator);
        self.rhs.print();
        print!("\n");
    }
}

pub struct Vivi {
    tokens: Vec<String>,
    curr: u32,
    ast: Vec<Box<dyn Expr>>,
    // operators: HashMap<String, Box<dyn Fn(&Vec<f32>) -> f32>>,
    operators: HashMap<String, i32>,
}

struct Operator {}

impl Vivi {
    /* Add table of operators here */
    pub fn new(query: &String) -> Self {
        let mut operators = HashMap::new();
        operators.insert("+".to_string(), 10);
        operators.insert("-".to_string(), 20);
        operators.insert("*".to_string(), 30);
        operators.insert("/".to_string(), 40);
        //operators.insert("+".to_string(), Operator::adder());
        //operators.insert("-".to_string(), Operator::subtractor());
        //operators.insert("*".to_string(), Operator::multiplier());
        Self {
            tokens: query.split_whitespace().map(|w| w.to_string()).collect(),
            curr: 0,
            ast: Vec::new(),
            operators: operators }
    }

    // x + 2x -> x ADD 2 MUL x
    pub fn eval(&mut self) {
        loop {
            if self.at_end() {
                break;
            }

            let expr = self.binary();
            self.ast.push(expr);   
        }
    }

    pub fn expr(&mut self, query: &String) {
        self.tokens = query.split_whitespace().map(|w| w.to_string()).collect();
        self.curr = 0;
        self.ast = Vec::new();
        
        self.eval();
    }

    pub fn derive(&self) {
        ()
    }

    pub fn display(&self) {
        for expr in &self.ast {
            expr.print();
        }
    }

    fn at_end(&self) -> bool {
        self.curr == self.tokens.len() as u32
    }

    fn binary(&mut self) -> Box<dyn Expr> {
        let mut lhs = self.primary();

        while self.is_operator(&self.peek()) {
            let operator = self.advance();
            let rhs = self.primary();
            lhs = Box::new(BinaryExpr::new(operator.to_string(), lhs, rhs));
        }
        return lhs;
    }

    fn primary(&mut self) -> Box<dyn Expr> {
        let curr_tok = self.peek();
        if is_variable(&curr_tok) {
            self.advance();
            return Box::new(LiteralExpr::new(None, Some(curr_tok)).unwrap());
        } else if is_number(&curr_tok) {
            self.advance();
            return Box::new(LiteralExpr::new(Some(curr_tok.parse::<f32>().unwrap()), None).unwrap());
        } else {
            return Box::new(LiteralExpr::new(None, None).unwrap());
        }
    }

    fn peek(&self) -> String {
        if !self.at_end() {
            return self.tokens[self.curr as usize].clone();
        }
        return "".to_string();
    }

    fn advance(&mut self) -> String {
        if !self.at_end() {
            let tok = self.tokens[self.curr as usize].clone();
            self.curr += 1;
            return tok;
        }
        return "".to_string();
    }

    fn is_operator(&self, x: &String) -> bool {
        match self.operators.get(x) {
            Some(_) => true,
            None => false
        }
    }

    // returns the operator from operator dictionary
    /*
    fn operator(&self, x: &str) -> Result<Box<dyn Fn(&Vec<f32>) -> f32>, &str> {
        match self.operators.get(x) {
            Some(op) => Ok(*op),
            None => Err("operation not found")
        }
    }
    */
    
    /*
    fn derive(&self, query: &str, var: &str) -> Expr {
        self.deriv(self.eval(query, var))
    }

    fn deriv(&self, expr: expr::Expr, var: &str) -> Expr {
        if is_number(expr) {
            return Expr::new(0);
        } else if is_variable(expr) {
            if same_variable(expr) {
                return expr::Expr::new(1);
            } else {
                return expr::Expr::new(0);
            }
        } else {
            let drules = derivation_rules(expr.operator());
            return drules(expr.operands(), var);
        }
    }
    */
}

/*
impl Operator {
    fn adder() -> Box<dyn Fn(&Vec<f32>) -> f32> {
        Box::new(move |nums: &Vec<f32>| {
            match nums.iter().reduce(|n, acc| &{n + acc}) {
                Some(n) => *n,
                None => 0.,
            }
        })
    }
    fn subtractor() -> Box<dyn Fn(&Vec<f32>) -> f32> {
        Box::new(move |nums: &Vec<f32>| {
            match nums.iter().reduce(|n, acc| &{n - acc}) {
                Some(n) => *n,
                None => 0.,
            }
        })
    }
    fn multiplier() -> Box<dyn Fn(&Vec<f32>) -> f32> {
        Box::new(move |nums: &Vec<f32>| {
            match nums.iter().reduce(|n, acc| &{n * acc}) {
                Some(n) => *n,
                None => 0.,
            }
        })
    }
}
*/

fn is_variable(tok: &String) -> bool {
    tok.chars().all(|c| c.is_alphabetic())
}

fn is_number(tok: &String) -> bool {
    tok.chars().all(|c| c.is_digit(10))
}