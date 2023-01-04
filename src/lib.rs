use std::{fmt, str::FromStr, collections::HashMap};
use fraction::{Fraction,Sign,ToPrimitive};
use num::pow;

pub mod derivative;

type Bank = HashMap<char, Letter>;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Equation {
    operation: Operation,
    element1: Box<Expression>,
    element2: Box<Expression>,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expression{
    Constant(Fraction),
    Variable(char),
    Equa(Box<Equation>),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Letter {
    Function(Value),
    Variable(Value),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Value {
    Defined(Expression),
    Undefined,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Operation { 
    Add,
    Sub,
    Mult,
    Div,
    Func, // when function element 1 is function name
    Exp,
    Trig, // element 1 denotes which func s,c,t with arc being caps (S,C,T)
    Log, // element 1 will represent base
    Deriv,
}

pub fn new_bank(function: char, input: &str) -> Bank {
    let mut bank: Bank = Bank::new();
    
    let i = 0; for character in input.chars() { 
        if character.is_alphabetic() { 
            if input.chars().nth(i+1) != Some('(') {
                bank.insert(character, Letter::Variable(Value::Undefined));
            } else if !(bank.contains_key(&character)) && input.chars().nth(i+1) == Some('(') {
                bank.insert(character, Letter::Function(Value::Undefined));
            }
        }
    }

    bank.insert(function, Letter::Function(Value::Defined(Expression::from(input))));

    bank
}


pub fn add_func_to_bank(bank: &mut Bank, f: char, input: &str) {
    bank.insert(f,Letter::Function(Value::Defined(Expression::from(input))));
}

pub fn add_var_to_bank(bank: &mut Bank, f: char, input: &str) {
    bank.insert(f,Letter::Variable(Value::Defined(Expression::from(input))));
}

pub fn get_expression(bank: &Bank, exp: &Expression) -> Expression {
    let exp_cpy = copy_expression(exp);

    // match for variable or equation
    match exp {
        Expression::Variable(v) => {
            let letter = bank.get(&v).unwrap();
            let fin = match letter {
                Letter::Function(Value::Defined(f)) => f,
                Letter::Variable(Value::Defined(v)) => v,
                _ => panic!("Probably undefined value"),
            };
            let fins = format!("{fin}");
            Expression::from(&fins)
        },
        Expression::Equa(e) => {
            if e.operation == Operation::Deriv {
                derivative::ddx(&*e.element1)
            } else {
                exp_cpy 
            }
        },
        _ => panic!("function, not Equa nor var🤨"),
    }
}

impl Expression {
    pub fn from(string: &str) -> Expression {
        //println!!("expression from {}", string);
        let mut i: usize = 0;
        let mut bracket_counter = 0;
        let mut last_add: usize = 0;
        let mut last_sub: usize = 0;
        let mut last_mult: usize = 0;
        let mut last_div: usize = 0;
        let mut last_func: usize = 0;
        let mut last_implied: usize = 0;
        let mut last_exp: usize = 0;
        let mut last_spec: usize = 0;
        let mut last_deriv: usize = 0;


        let first = string.chars().nth(0).unwrap();
        let last  = string.chars().nth(string.len()-1).unwrap();

        let string = if (first,last) == ('(',')') {
            if is_wrapped(string) {&string[1..string.len()-1]}else{string}
        } else {
            string
        };


        let mut last_char = '(';
        let mut during_name = false;
        // find operators
        for character in string.chars() {
            if bracket_counter == 0 {
                match character {
                    '+' => last_add  = i, 
                    '-' => last_sub  = i,
                    '*' => last_mult = i,
                    '/' => last_div  = i,
                    '^' => last_exp  = i,
                    '(' => { if last_char.is_alphabetic() { last_func = i }
                            bracket_counter += 1 
                            },
                   '\'' => last_deriv = i,
                    _   => (),
                }
                // a(x) needs to be counted as a function, even if it is a*x it will be handled later
                if is_implied_mult(last_char, character) && !is_func(last_char, character)  { 
                    last_implied = i 
                }
                if is_special(string, i) {
                    during_name = true;
                    bracket_counter -= 1
                }
            } else {
                match character {
                    '(' => { bracket_counter += 1;
                             if during_name {
                                 during_name = false;
                                 last_spec = i;
                                 bracket_counter +=1; // inside the name is skipped via brackets
                             }
                           },
                    ')' => bracket_counter -= 1,
                    _   => (),
                }
            }

            i+=1;
            last_char = character;
        }
        
            
        // find op to use SAMDEB order
        if last_add != 0 {
            ////println!!("{} + {}",&string[..last_add],&string[last_add+1..]);
            Expression::Equa(Box::new(Equation {
                operation: Operation::Add,
                element1: Box::new(Expression::from(&string[..last_add])),
                element2: Box::new(Expression::from(&string[last_add+1..])),
            }))
        } else if last_sub != 0 {
            Expression::Equa(Box::new(Equation {
                operation: Operation::Sub,
                element1: Box::new(Expression::from(&string[..last_sub])),
                element2: Box::new(Expression::from(&string[last_sub+1..])),
            }))
        } else if last_mult != 0 {
            Expression::Equa(Box::new(Equation {
                operation: Operation::Mult,
                element1: Box::new(Expression::from(&string[..last_mult])),
                element2: Box::new(Expression::from(&string[last_mult+1..])),
            }))
        } else if last_div != 0 {
            Expression::Equa(Box::new(Equation {
                operation: Operation::Div,
                element1: Box::new(Expression::from(&string[..last_div])),
                element2: Box::new(Expression::from(&string[last_div+1..])),
            }))
        } else if last_exp != 0 {
            Expression::Equa(Box::new(Equation {
                operation: Operation::Exp,
                element1: Box::new(Expression::from(&string[..last_exp])),
                element2: Box::new(Expression::from(&string[last_exp+1..])),
            }))
        } else if last_implied != 0 {
            let x = &string[..last_implied];
            let y = &string[last_implied..];
            Expression::Equa(Box::new(Equation {
                operation: Operation::Mult,
                element1: Box::new(Expression::from(&string[..last_implied])),
                element2: Box::new(Expression::from(&string[last_implied..])),
            }))
        } else if last_func != 0 { 
            Expression::Equa(Box::new(Equation {
                operation: Operation::Func,
                element1: Box::new(Expression::from(&string[..last_func])),
                element2: Box::new(Expression::from(&string[last_func..])),
            }))
        } else if last_spec != 0 {
            let mut trig = true;
            
            let name = match &string[..last_spec] {
                "sin" => "s",
                "cos" => "c",
                "tan" => "t",
                "arcsin" => "S",
                "arccos" => "C",
                "arctan" => "T",
                "ln" => {trig=false; "e"}
                "log" => {trig=false; "10"}
                s if s.starts_with("log") => {trig=false; &s[4..]}
                _ => panic!("last spec but doesnt match")
            };
            if trig { Expression::Equa(Box::new(Equation {
                operation: Operation::Trig,
                element1: Box::new(Expression::from(name)),
                element2: Box::new(Expression::from(&string[last_spec..])),
                }))
            } else { Expression::Equa(Box::new(Equation {
                operation: Operation::Log,
                element1: Box::new(Expression::from(name)),
                element2: Box::new(Expression::from(&string[last_spec..])),
                }))
            }
        } else if last_deriv != 0 {
            //println!!("{} is a deriv",&string[..last_deriv]);
            Expression::Equa(Box::new(Equation {
                operation: Operation::Deriv,
                element1: Box::new(Expression::from(&string[..last_deriv])),
                element2: Box::new(Expression::Variable('!')), // element 2 is meaningless for deriv
            }))
        } else {
           let as_frac = Fraction::from_str(string);

           match as_frac {
               Ok(frac) => Expression::Constant(frac),
               Err(error) => {
                   Expression::Variable(get_name(string))
               },
           }
        }             
    } 

    pub fn evaluate(&self, bank: &Bank, x: Fraction) -> Fraction {
        if let Expression::Equa(equation) = self {
            return match &equation.operation {
                Operation::Add => 
                    equation.element1.evaluate(bank, x) + equation.element2.evaluate(bank, x),
                Operation::Sub => 
                    equation.element1.evaluate(bank, x) - equation.element2.evaluate(bank, x),
                Operation::Mult => 
                    equation.element1.evaluate(bank, x) * equation.element2.evaluate(bank, x),
                Operation::Div => 
                    equation.element1.evaluate(bank, x) / equation.element2.evaluate(bank, x),
                Operation::Exp =>
                    power(equation.element1.evaluate(bank, x), equation.element2.evaluate(bank, x)),
                Operation::Func => {
                    get_expression(bank,&equation.element1).evaluate(bank,x)
                    //if let Expression::Variable(f) = *equation.element1 {
                    //        match bank.get(&f).unwrap() {
                    //            Letter::Function(Value::Defined(value)) => 
                    //                value.evaluate(bank, equation.element2.evaluate(bank, x)),
                    //            Letter::Variable(Value::Defined(value)) =>
                    //                value.evaluate(bank, x) * equation.element2.evaluate(bank, x),
                    //            _ => panic!("UNDEFINED"),
                    //            }
                    //} else {
                    //    panic!("UH OH")
                    //}
                },
                Operation::Trig => {
                    solve_trig(&equation.element1, equation.element2.evaluate(bank, x))
                    },
                Operation::Log => {
                    solve_log(&equation.element1, equation.element2.evaluate(bank, x), bank, x)
                    },
                Operation::Deriv => {
                    let element1_cpy = copy_expression(&equation.element1);
                    derivative::eval_deriv(element1_cpy, bank, x).evaluate(bank, x)
                }
                _   => panic!("No OP BUT EQUATION"),
            }
        } else {
            match self {
                Expression::Constant(constant) => *constant,
                Expression::Variable('x')      => x,
                Expression::Variable(name)     => if let Letter::Variable(Value::Defined(v)) = bank.get(&name).unwrap() {
                                                    v.evaluate(bank, x)  
                                                  } else {
                                                    panic!("TRYING TO FIND UNDEFIEND VALUE")
                                                  }
                _ => panic!("ln:209ish\nsomethings messed up\nIN EVALUATE")
                }
        }
    }
}

impl Letter {
    pub fn get_inside(&self) -> &Value {
        return match self {
            Letter::Function(x) => x,
            Letter::Variable(x) => x,
        }
    }
    pub fn evaluate(&self, bank: &Bank, x: Fraction) -> Fraction {
        let inside = self.get_inside();
        if let Value::Defined(value) = inside {
            value.evaluate(bank, x)
        } else {
            Fraction::from(0)
        }
    }
    pub fn get_expression(&self) -> &Expression {
        let val = self.get_inside();

        if let Value::Defined(expression) = val {
            expression
        } else {
            panic!("undefined")
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return if let Expression::Equa(equation) = self {
            match &equation.operation {
                Operation::Add => write!(f,"({})+({})",*equation.element1,*equation.element2),
                Operation::Sub => write!(f,"({})-({})",*equation.element1,*equation.element2),
                Operation::Mult => write!(f,"({})({})",*equation.element1,*equation.element2), //×
                Operation::Div => write!(f,"({})/({})",*equation.element1,*equation.element2),
                Operation::Func => write!(f,"({}({}))",*equation.element1,*equation.element2),
                Operation::Exp => write!(f,"({}^({}))",*equation.element1,*equation.element2),
                Operation::Trig => {
                    if let Expression::Variable(name) = *equation.element1 {
                        match name {
                            's' => write!(f,"(sin({}))",*equation.element2),
                            'c' => write!(f,"(cos({}))",*equation.element2),
                            't' => write!(f,"(tan({}))",*equation.element2),
                            'S' => write!(f,"(arcsin({}))",*equation.element2),
                            'C' => write!(f,"(arccos({}))",*equation.element2),
                            'T' => write!(f,"(arctan({}))",*equation.element2),
                            _ => write!(f,"NO trig name display"),
                        }
                    } else {
                        write!(f,"trig name is invalid")
                    }},
                Operation::Log => {
                    let ten = Fraction::from(10);
                    match *equation.element1 {
                        Expression::Variable('e') => write!(f,"(ln({}))",*equation.element2),
                        Expression::Variable(v) => write!(f,"(log{}({}))",v,*equation.element2),
                        Expression::Constant(c) => { if c == ten { 
                                                        write!(f,"(log({}))",*equation.element2)
                                                    } else { 
                                                        write!(f,"(log{}({}))",c,*equation.element2)
                                                    }
                        },
                        _ => panic!("something is up in the log section of display"),
                    }},
                Operation::Deriv => write!(f,"{}'",*equation.element1),
                 _ => panic!("No OP BUT EQUATION"),
            }
        } else {
            match self {
                Expression::Constant(constant) => write!(f,"{}",*constant),
                Expression::Variable(variable) => write!(f,"{}",*variable),
                _ => panic!("NOT CONSTANT OR VAR BUT NOT Equation"),
            }
        }
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inside = self.get_inside();
        match inside {
            Value::Defined(value) => write!(f,"{}",value),
            Value::Undefined => write!(f,"THIS IS UNDEFINED"),
        }
    }
}

fn is_implied_mult(before: char, current: char) -> bool {
    let mut both_number = true;
    for character in [before,current] {
        match character {
            '+' | '-' | '*' | '/' | '^' | '\'' => return false, 
            _ => ()
        }
        both_number = both_number && character.is_numeric();
    }
    !both_number && before != '(' && current != ')'
}

fn is_func(before: char, current: char) -> bool {
    (current == '(') && (before.is_alphabetic() | (before == '\'') )
}

fn is_special(text: &str, start: usize) -> bool {
    if text.len()-start >= 6 {
        match &text[start..start+6] {
            "arcsin" | "arctan" | "arccos" => return true,
            _ => (),
        };
        match &text[start..start+3] {
            "cos" | "tan" | "sin" | "log" => return true,
            _ => (),
        };
    } else if text.len()-start >= 4 {
        match &text[start..start+3] {
            "cos" | "tan" | "sin" | "log" => return true,
            _ => (),
        };
        match &text[start..start+2] {
            "ln" => return true,
            _ => (),
        };
    } else if text.len()-start >= 3 {
        match &text[start..start+2] {
            "ln" => return true,
            _ => (),
        };
    }
    return false;
}

fn get_name(string: &str) -> char {
    //println!!("string: {}\nlen: {}", string, string.len());
    assert_eq!(1, string.len());
    let name = string.clone();
    let name = name.chars().nth(0).unwrap();
    name
}

fn power(base: Fraction, exponent: Fraction) -> Fraction {
    let base = base.to_f64().unwrap();
    let exponent = exponent.to_f64().unwrap();

    let answer = base.powf(exponent);
    Fraction::from(answer)
}

fn solve_trig(name: &Expression, arg: Fraction) -> Fraction {
    let arg = arg.to_f64().unwrap();

    let answer = if let &Expression::Variable(n) = name {
        match n {
            's' => arg.sin(),
            'c' => arg.cos(),
            't' => arg.tan(),
            'S' => arg.sin().asin(),
            'C' => arg.cos().acos(),
            'T' => arg.tan().atan(),
            _ => panic!("tried to find trig function {}",n),
        }
    } else {
        panic!("NAME IS {} NOT A Expression::Variable",name)
    };
    Fraction::from(answer)
}

fn solve_log(base: &Expression, arg: Fraction, bank: &Bank, x: Fraction) -> Fraction {
    let arg = arg.to_f64().unwrap();

    let answer = if let &Expression::Variable(e) = base {
        if e == 'e' {arg.ln()}
        else {
            let base = base.evaluate(bank, x).to_f64().unwrap();
            arg.log(base)
        }
    } else {
        let base = base.evaluate(bank, x).to_f64().unwrap();
        arg.log(base)
    };
    Fraction::from(answer)
}

fn is_wrapped(s: &str) -> bool {
    let not_last = &s[..s.len()-1];
    //println!!("not last: {}",not_last);

    let mut bracket_counter = 0; 
    for c in not_last.chars() {
        match c {
            '(' => bracket_counter -= 1,
            ')' => bracket_counter += 1,
            _   => ()
        }
        if bracket_counter == 0 {
            return false;
        }
    } 
    true 
}

pub fn copy_expression(e: &Expression) -> Expression {
    let e_str = format!("{}",e);
    Expression::from(&e_str)
}

