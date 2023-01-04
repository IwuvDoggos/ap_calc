use std::{fmt, str::FromStr, cell::RefCell, collections::HashMap, rc::Rc};
use fraction::{Fraction,Sign};

#[derive(Debug)]
pub struct Equation<'a> {
    operation: Operation,
    element1: Box<Expression<'a>>,
    element2: Box<Expression<'a>>,
    bank: Rc<RefCell<HashMap<char, Variable<'a>>>>}

#[derive(Debug)]
pub enum Expression<'a>{
    Constant(Fraction),
    Variable(&'a str),
    Equa(Box<Equation<'a>>),
}

#[derive(Debug)]
pub enum Variable<'a> {
    Function(Value<'a>),
    Var(Value<'a>),
}

#[derive(Debug)]
pub enum Value<'a> {
    Defined(Expression<'a>),
    Undefined,
}

impl Expression<'_> {
    pub fn from<'a>(string: &'a str,bank: Rc<RefCell<HashMap<char, Variable<'a>>>>) -> Expression<'a> {
        let mut i: usize = 0;
        let mut bracket_counter = 0;
        let mut last_add: usize = 0;
        let mut last_sub: usize = 0;
        let mut last_mult: usize = 0;
        let mut last_div: usize = 0;
        let mut last_func: usize = 0;
        let mut last_implied: usize = 0;

        let first = string.chars().nth(0).unwrap();
        let last  = string.chars().nth(string.len()-1).unwrap();

        let string = if (first,last) == ('(',')') {
            &string[1..string.len()-1]
        } else {
            string
        };


        let mut last_char = ' ';
        // find operators
        for character in string.chars() {
            if bracket_counter == 0 {
                match character {
                    '+' => last_add  = i, 
                    '-' => last_sub  = i,
                    '*' => last_mult = i,
                    '/' => last_div  = i,
                    '(' => { if i != 0 {
                                if string.chars().nth(i-1).unwrap().is_alphabetic() {
                            }
                        } bracket_counter += 1 },
                    _   => ()
            } else {
                match character {
                    '(' => bracket_counter += 1,
                    ')' => bracket_counter -= 1,
                    _   => (),
                }
            }
            i+=1;
            last_char = character;
        }
            
        // find op to use SAMDEB order
        if last_add != 0 {
            //println!("{} + {}",&string[..last_add],&string[last_add+1..]);
            Expression::Equa(Box::new(Equation {
                operation: Operation::Add,
                element1: Box::new(Expression::from(&string[..last_add],Rc::clone(&bank))),
                element2: Box::new(Expression::from(&string[last_add+1..],Rc::clone(&bank))),
                bank: Rc::clone(&bank),
            }))
        } else if last_sub != 0 {
            Expression::Equa(Box::new(Equation {
                operation: Operation::Sub,
                element1: Box::new(Expression::from(&string[..last_sub],Rc::clone(&bank))),
                element2: Box::new(Expression::from(&string[last_sub+1..],Rc::clone(&bank))),
                bank: Rc::clone(&bank),
            }))
        } else if last_mult != 0 {
            Expression::Equa(Box::new(Equation {
                operation: Operation::Mult,
                element1: Box::new(Expression::from(&string[..last_mult],Rc::clone(&bank))),
                element2: Box::new(Expression::from(&string[last_mult+1..],Rc::clone(&bank))),
                bank: Rc::clone(&bank),
            }))
        } else if last_div != 0 {
            Expression::Equa(Box::new(Equation {
                operation: Operation::Div,
                element1: Box::new(Expression::from(&string[..last_div],Rc::clone(&bank))),
                element2: Box::new(Expression::from(&string[last_div+1..],Rc::clone(&bank))),
                bank: Rc::clone(&bank),
            }))
        } else {
           let as_frac = Fraction::from_str(string);

           match as_frac {
               Ok(frac) => Expression::Constant(frac),
               Err(error) => Expression::Variable(&string),
           }
        }             
    }

    pub fn evaluate(&self, x: i32) -> Fraction {
        if let Expression::Equa(equation) = self {
            return match &equation.operation {
                Operation::Add => equation.element1.evaluate(x) + equation.element2.evaluate(x),
                Operation::Sub => equation.element1.evaluate(x) - equation.element2.evaluate(x),
                Operation::Mult => equation.element1.evaluate(x) * equation.element2.evaluate(x),
                Operation::Div => equation.element1.evaluate(x) / equation.element2.evaluate(x),
                _   => panic!("No OP BUT EQUATION"),
            }
        } else {
            if let Expression::Constant(constant) = self {
                constant
            } else {
                Fraction::from(x)
            }
        }
    }
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return if let Expression::Equa(equation) = self {
            let symbol = match &equation.operation {
                Operation::Add => "+",
                Operation::Sub => "-",
                Operation::Mult => "×",
                Operation::Div => "/",
                _   => panic!("No OP BUT EQUATION"),
            };
            write!(f,"({}){}({})",*equation.element1,symbol,*equation.element2)
        } else {
            if let Expression::Constant(constant) = self {
                write!(f,"{}",*constant)
            } else { 
                if let Expression::Variable(variable) = self {
                    write!(f,"{}",*variable)
                } else {
                    panic!("IS VARIABLE");
                }
            }
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Function, // when function element 1 is function name
}
