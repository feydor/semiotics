// use crate::vivi::expr;
use std::collections::HashMap;
use std::process;
use std::rc::Rc;

trait Expr {
    fn print(&self);
    fn simplify(&self) -> Box<LiteralExpr>;

    fn is_constant(&self, var: &str) -> bool;
    fn is_same_variable(&self, var: &str) -> bool;
    fn is_sum(&self, var: &str) -> bool;
    fn is_product(&self, var: &str) -> bool;

    fn number(&self) -> f32;
    fn variable(&self) -> &str;
    fn augend(&self) -> Rc<dyn Expr>;
    fn addend(&self) -> Rc<dyn Expr>;
    fn multiplicand(&self) -> Rc<dyn Expr>;
    fn multiplier(&self) -> Rc<dyn Expr>;
}

struct BinaryExpr {
    lhs: Rc<dyn Expr>,
    rhs: Rc<dyn Expr>,
    operator: Operator,
}

#[derive(Default, Clone)]
struct LiteralExpr {
    value: Value,
    is_number: bool,
    is_variable: bool,
}

#[derive(Default, Clone)]
struct Value {
    num: f32,
    var: String,
}

enum Operator {
    PLUS,
    MINUS,
    STAR,
    SLASH
}

impl Value {
    fn new_num(n: f32) -> Self {
        Self { num: n, var: "".to_string() }
    }

    fn new_var(var: &str) -> Self {
        Self { num: 0., var: var.to_string() }
    }
}

impl LiteralExpr {
    fn new_num(n: f32) -> Self {
        Self { value: Value::new_num(n), is_number: true, is_variable: false }
    }

    fn new_var(var: &str) -> Self {
        Self { value: Value::new_var(var), is_number: false, is_variable: true }
    }
}

impl Expr for LiteralExpr {
    fn print(&self) {
        match self {
            LiteralExpr {is_number: true, .. } => print!("{} ", self.value.num),
            LiteralExpr {is_variable: true, .. } => print!("{} ", self.value.var),
            _ => print_error_and_exit("Not a valid literal")
        }
    }

    // TODO: FINISH
    fn simplify(&self) -> Box<LiteralExpr> {
        Box::new(self.clone())
    }

    fn is_constant(&self, _var: &str) -> bool {
        return self.is_number;
    }

    fn is_same_variable(&self, var: &str) -> bool {
        return self.is_variable && self.value.var == var;
    }

    fn is_sum(&self, _var: &str) -> bool {
        false
    }

    fn is_product(&self, _var: &str) -> bool {
        false
    }

    // TODO: It doesn't make sense for a LiteralExpr to have:
    // addend(), augend(), multiplicand(), multiplier()
    fn augend(&self) -> Rc<dyn Expr> {
        return Rc::new(LiteralExpr::new_num(0.));
    }

    fn addend(&self) -> Rc<dyn Expr> {
        return Rc::new(LiteralExpr::new_num(0.));
    }

    fn multiplicand(&self) -> Rc<dyn Expr> {
        return Rc::new(LiteralExpr::new_num(0.));
    }

    fn multiplier(&self) -> Rc<dyn Expr> {
        return Rc::new(LiteralExpr::new_num(0.));
    }

    fn number(&self) -> f32 {
        self.value.num
    }

    fn variable(&self) -> &str {
        &self.value.var
    }

}

impl BinaryExpr {
    fn new(operator: Operator, lhs: Rc<dyn Expr>, rhs: Rc<dyn Expr>) -> Self {
        Self { operator: operator, lhs: lhs, rhs: rhs }
    }

    fn apply_operator(&self, lhs: Box<LiteralExpr>, rhs: Box<LiteralExpr>) -> f32 {
        match self.operator {
            Operator::PLUS => lhs.value.num + rhs.value.num,
            Operator::STAR => lhs.value.num * rhs.value.num,
            Operator::MINUS => lhs.value.num - rhs.value.num,
            Operator::SLASH => lhs.value.num / rhs.value.num,
        }
    }
}

impl Expr for BinaryExpr {
    fn print(&self) {
        self.lhs.print();
        self.operator.print();
        self.rhs.print();
    }

    // TODO: FINISH
    fn simplify(&self) -> Box<LiteralExpr> {
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();


        match self.operator {
            Operator::PLUS => Box::new(LiteralExpr::new_num(lhs.value.num + rhs.value.num)),
            Operator::STAR => Box::new(LiteralExpr::new_num(lhs.value.num * rhs.value.num)),
            Operator::MINUS => Box::new(LiteralExpr::new_num(lhs.value.num - rhs.value.num)),
            Operator::SLASH => Box::new(LiteralExpr::new_num(lhs.value.num / rhs.value.num)),
        }
    }

    fn is_constant(&self, var: &str) -> bool {
        // if both lhs and rhs are constants, then the result is a constant
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();

        lhs.is_constant(&var) && rhs.is_constant(&var)
    }

    fn is_same_variable(&self, _var: &str) -> bool {
        false
    }

    fn is_sum(&self, _var: &str) -> bool {
        match self.operator {
            Operator::PLUS => true,
            _ => false
        }
    }

    fn is_product(&self, _var: &str) -> bool {
        match self.operator {
            Operator::STAR => true,
            _ => false
        }
    }

    fn augend(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.lhs)
    }

    fn addend(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.rhs)
    }

    fn multiplicand(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.lhs)
    }

    fn multiplier(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.rhs)
    }

    // TODO: Use simplify
    fn number(&self) -> f32 {
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();

        return self.apply_operator(lhs, rhs);
    }

    // TODO: Use simplify
    fn variable(&self) -> &str {
        "PYTHIA"
    }
}

impl Operator {
    fn from_string(s: &str) -> Operator {
        match s {
            "+" => Operator::PLUS,
            "-" => Operator::MINUS,
            "*" => Operator::STAR,
            "/" => Operator::SLASH,
            _ => panic!("{}", "UNKNOWN OPERATOR: ".to_string() + s),
        }
    }

    fn print(&self) {
        match self {
            Operator::PLUS => print!("+"),
            Operator::MINUS => print!("-"),
            Operator::STAR => print!("*"),
            Operator::SLASH => print!("/"),
        }
        print!(" ");
    }
}

pub struct Vivi {
    tokens: Vec<String>,
    curr: u32,
    ast: Vec<Rc<dyn Expr>>,
    // operators: HashMap<String, Box<dyn Fn(&Vec<f32>) -> f32>>,
    operators: HashMap<String, i32>,
}

impl Vivi {
    /* Add table of operators here */
    pub fn new(query: &String) -> Self {
        let mut operators = HashMap::new();
        operators.insert("+".to_string(), 10);
        operators.insert("-".to_string(), 20);
        operators.insert("*".to_string(), 30);
        operators.insert("/".to_string(), 40);
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

    // parses the given query
    pub fn parse_expr(&mut self, query: &String) {
        self.tokens = query.split_whitespace().map(|w| w.to_string()).collect();
        self.curr = 0;
        self.ast = Vec::new();
        
        self.eval();
    }

    pub fn simplify(&mut self) {
        self.eval();
        for expr in &self.ast {
            let e = *expr.simplify();
            e.print();
            print!("\n");
        }
    }

    pub fn differentiate(&mut self, var: &str) {
        self.eval();
        let v = self.ast.remove(0);
        println!("var: {}", var);
        let derived = self.deriv(v, &var);
        derived.print();
    }

    pub fn display(&self) {
        for expr in &self.ast {
            expr.print(); 
        }
        print!("\n");
    }

    fn deriv(&self, expr: Rc<dyn Expr>, var: &str) -> Rc<dyn Expr> {
        if expr.is_constant(&var) {
            return Rc::new(LiteralExpr::new_num(0.));
        } else if expr.is_same_variable(&var) {
            return Rc::new(LiteralExpr::new_num(1.));
        } else if expr.is_sum(&var) {
            return self.make_sum(
                self.deriv(expr.augend(), &var),
                self.deriv(expr.addend(), &var),
                &var
            );
        } else if expr.is_product(&var) {
            return self.make_sum(
                self.make_product(expr.multiplier(), self.deriv(expr.multiplicand(), &var), &var),
                self.make_product(self.deriv(expr.multiplier(), &var), expr.multiplicand(), &var),
                &var
            );
        } else {
            panic!("Vivi::deriv: Unkown differentation rule.");
        }
    }
/*
    (define (make-sum a1 a2)
  (cond [(and (number? a1) (number? a2))
         (+ a1 a2)]
        [(=number? a1 0) a2]
        [(=number? a2 0) a1]
        [else (list '+ a1 a2)]))
*/

    fn make_sum(&self, lhs: Rc<dyn Expr>, rhs: Rc<dyn Expr>, var: &str) -> Rc<dyn Expr> {
        if lhs.is_constant(&var) && rhs.is_constant(&var) {
            return Rc::new(LiteralExpr::new_num(lhs.number() + rhs.number()));
        } else if lhs.is_constant(&var) && lhs.number() == 0. {
            return rhs;
        } else if rhs.is_constant(&var) && rhs.number() == 0. {
            return lhs;
        }

        Rc::new(BinaryExpr::new(Operator::PLUS, lhs, rhs))
    }

    fn make_product(&self, lhs: Rc<dyn Expr>, rhs: Rc<dyn Expr>, var: &str) -> Rc<dyn Expr> {
        if lhs.is_constant(&var) && rhs.is_constant(&var) {
            return Rc::new(LiteralExpr::new_num(lhs.number() * rhs.number()));
        } else if lhs.is_constant(&var) && lhs.number() == 1. {
            return Rc::new(LiteralExpr::new_var(rhs.variable()));
        } else if rhs.is_constant(&var) && rhs.number() == 1. {
            return Rc::new(LiteralExpr::new_var(lhs.variable()));
        } else if lhs.is_constant(&var) && lhs.number() == 0. {
            return Rc::new(LiteralExpr::new_num(0.));
        } else if rhs.is_constant(&var) && rhs.number() == 0. {
            return Rc::new(LiteralExpr::new_num(0.));
        }
        Rc::new(BinaryExpr::new(Operator::STAR, lhs, rhs))
    }

    fn at_end(&self) -> bool {
        self.curr == self.tokens.len() as u32
    }

    fn binary(&mut self) -> Rc<dyn Expr> {
        let mut lhs = self.primary();

        while self.is_operator(&self.peek()) {
            let operator = self.advance();
            let rhs = self.primary();
            lhs = Rc::new(BinaryExpr::new(Operator::from_string(&operator), lhs, rhs));
        }
        return lhs;
    }

    fn primary(&mut self) -> Rc<dyn Expr> {
        let curr_tok = self.peek();
        if is_variable(&curr_tok) {
            self.advance();
            return Rc::new(LiteralExpr::new_var(&curr_tok));
        } else if is_number(&curr_tok) {
            self.advance();
            return Rc::new(LiteralExpr::new_num(curr_tok.parse::<f32>().unwrap()));
        } else {
            print_error_and_exit(&self.peek());
            process::exit(1);
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
}

fn is_variable(tok: &String) -> bool {
    tok.chars().all(|c| c.is_alphabetic())
}

fn is_number(tok: &String) -> bool {
    tok.chars().all(|c| c.is_digit(10))
}

fn print_error_and_exit(err: &str) {
    println!("Unknown token: {}", err);
    process::exit(1);
}