// use crate::vivi::expr;

#[allow(dead_code)]
pub struct Vivi {
    tokens: Vec<String>,
}

impl Vivi {
    pub fn make_sum(&self, a1: Vec<String>, a2: Vec<String>) -> Vec<String> {
        if is_number(&a1) && is_number(&a2) {
            let sum = parse_number(&a1) + parse_number(&a2);
            return vec![sum.to_string()]
        } else if is_number_and_equals(&a1, 0.) {
            return a2.to_vec();
        } else if is_number_and_equals(&a2, 0.) {
            return a1.to_vec();
        } else {
            let temp:Vec<String> = concat_vectors(&a1, &vec!["+".to_string()]);
            return concat_vectors(&temp, &a2);
        }
    }
    
    pub fn make_product(&self, m1: Vec<String>, m2: Vec<String>) -> Vec<String> {
        if is_number(&m1) && is_number(&m2) {
            let product = parse_number(&m1) * parse_number(&m2);
            return vec![product.to_string()];
        } else if is_number_and_equals(&m1, 1.) {
            return m2.to_vec();
        } else if is_number_and_equals(&m2, 1.) {
            return m1.to_vec();
        } else if is_number_and_equals(&m1, 0.) || is_number_and_equals(&m2, 0.) {
            return self.make_number(0.);
        } else {
            let temp = concat_vectors(&m1, &vec!["*".to_string()]);
            return concat_vectors(&temp, &m2);
        }
    }

    fn make_number(&self, num: f32) -> Vec<String>{
        return vec![num.to_string()];
    }

    pub fn differentiate(&self, query: &str, variable: &str) {
        let expr: Vec<String> = query.split_whitespace().map(|w| w.to_string()).collect();
        let derived = self.deriv(expr.clone(), &variable);
        println!("{:?}", derived);
    }

    fn deriv(&self, expr: Vec<String>, var: &str) -> Vec<String> {
        if is_constant(&expr, &var) {
            return self.make_number(0.);
        } else if is_same_variable(&expr, &var) {
            return self.make_number(1.)
        } else if is_sum(&expr, &var) {
            return self.make_sum(
                self.deriv(augend(&expr).to_vec(), &var),
                self.deriv(addend(&expr).to_vec(), &var)
            );
        } else if is_product(&expr, &var) {
            return self.make_sum(
                self.make_product(multiplicand(&expr), self.deriv(multiplier(&expr).to_vec(), &var)),
                self.make_product(self.deriv(multiplicand(&expr).to_vec(), &var), multiplier(&expr))
            );
        } else {
            panic!("{}", "INVALID DIFFERENTIATION");
        }
    }
    
    /* Add table of operators here */
    pub fn new(query: &String) -> Self {
        Self {
            tokens: query.split_whitespace().map(|w| w.to_string()).collect(),
        }
    }

}


fn concat_vectors<T: Clone>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    v1.iter().cloned().chain(v2.iter().cloned()).collect()
}

// a constant or a variable, not a list
fn is_atomic(v: &Vec<String>) -> bool {
    v.len() == 1 as usize
}

fn is_number(n: &Vec<String>) -> bool {
    is_atomic(n) && n[0].chars().all(|c| c.is_digit(10))
}

fn is_number_and_equals(n: &Vec<String>, test: f32) -> bool {
    is_number(n) && n[0].parse::<f32>().unwrap() == test
}

fn parse_number(n: &Vec<String>) -> f32 {
    n[0].parse().unwrap()
}

fn is_constant(expr: &Vec<String>, var: &str) -> bool {
    is_atomic(&expr) && expr[0] != var
}

fn is_same_variable(expr: &Vec<String>, var: &str) -> bool {
    is_atomic(&expr) && expr[0] == var
}

fn is_sum(expr: &Vec<String>, _var: &str) -> bool {
    !is_atomic(&expr) && expr[1] == "+"
}

fn is_product(expr: &Vec<String>, _var: &str) -> bool {
    !is_atomic(&expr) &&  expr[1] == "*"//expr.iter().any(|subexp| subexp == "*")
}

fn augend(expr: &Vec<String>) -> Vec<String> {
    vec![expr[0].clone()]
}

fn addend(expr: &Vec<String>) -> Vec<String> {
    vec![expr[2].clone()]
}

fn multiplicand(expr: &Vec<String>) -> Vec<String> {
    vec![expr[0].clone()]
}

fn multiplier(expr: &Vec<String>) -> Vec<String> {
    vec![expr[2].clone()]
}