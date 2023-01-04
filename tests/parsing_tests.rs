use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use fraction::{Fraction,Sign};

#[test]
fn make_a_equation() {
    let wanted: &str = "4+2*(2-30)2+40+x";
    let mut bank = ap_calc::new_bank('f', wanted);
}

#[test]
fn test_implied() {
    let mut bank = ap_calc::new_bank('f', "2(2-30)2");

    let f_of_5 = bank.get(&'f').unwrap().evaluate(&bank, Fraction::from(5)); 
    println!("f(x) = {}", bank.get(&'f').unwrap());
    println!("f(5) = {}", bank.get(&'f').unwrap().evaluate(&bank, Fraction::from(5)));
}

#[test]
fn adding_to_bank() {
    let mut bank = ap_calc::new_bank('f',"3x+25");

    ap_calc::add_func_to_bank(&mut bank, 'g', "30(5x-20)-5");

    println!("g(x) = {}", bank.get(&'g').unwrap());
    println!("g(5) = {}", bank.get(&'g').unwrap().evaluate(&bank, Fraction::from(5)));
}

#[test]
fn test_functions() {
    let mut bank = ap_calc::new_bank('f',"3x+g(5)");

    ap_calc::add_func_to_bank(&mut bank, 'g', "30(5x-20)-5");

    println!("f(x) = {}", bank.get(&'f').unwrap());
    println!("f(5) = {}", bank.get(&'f').unwrap().evaluate(&bank, Fraction::from(5)));
}

#[test]
fn test_variables() {
    let mut bank = ap_calc::new_bank('f',"2x+a");


    ap_calc::add_var_to_bank(&mut bank, 'a', "b+c");
    ap_calc::add_var_to_bank(&mut bank, 'b', "2d");
    ap_calc::add_var_to_bank(&mut bank, 'c', "5");
    ap_calc::add_var_to_bank(&mut bank, 'd', "10");

    println!("f(x) = {}", bank.get(&'f').unwrap());
    println!("f(5) = {}", bank.get(&'f').unwrap().evaluate(&bank, Fraction::from(5)));

    ap_calc::add_var_to_bank(&mut bank, 'c', "3d");

    println!("f(5) with when c=3d = {}", bank.get(&'f').unwrap().evaluate(&bank, Fraction::from(5)));
}


#[test]
fn test_exponents() { let mut bank = ap_calc::new_bank('f',"x^2");
    println!("f(x) = {}", bank.get(&'f').unwrap());
    println!("f(5) = {}", bank.get(&'f').unwrap().evaluate(&bank, Fraction::from(5)));
}

#[test]
fn test_trig_funcs() {
    let mut bank = ap_calc::new_bank('c',"cos(x)");
    ap_calc::add_func_to_bank(&mut bank, 's', "sin(x)");
    ap_calc::add_func_to_bank(&mut bank, 't', "tan(x)");
    ap_calc::add_func_to_bank(&mut bank, 'C', "arccos(x)");
    ap_calc::add_func_to_bank(&mut bank, 'S', "arcsin(x)");
    ap_calc::add_func_to_bank(&mut bank, 'T', "arctan(x)");

    println!("cos(x) = {}", bank.get(&'c').unwrap());
    println!("arccos(x) = {}", bank.get(&'C').unwrap());
    println!("cos(5) = {}", bank.get(&'c').unwrap().evaluate(&bank, Fraction::from(5)));
    println!("arccos(5) = {}", bank.get(&'C').unwrap().evaluate(&bank, Fraction::from(5)));
}

#[test]
fn test_logs() {
    let mut bank = ap_calc::new_bank('l',"ln(x)");
    ap_calc::add_func_to_bank(&mut bank, 'L', "log(x)");
    
    println!("ln(x) = {}", bank.get(&'l').unwrap());
    println!("ln(5) = {}", bank.get(&'l').unwrap().evaluate(&bank, Fraction::from(5)));
    println!("log(5) = {}", bank.get(&'L').unwrap().evaluate(&bank, Fraction::from(5)));
}

#[test]
fn test_third() {
    // worried that because 1/3 is infentessimal
    let mut bank = ap_calc::new_bank('f',"x/3");

    ap_calc::add_func_to_bank(&mut bank, 's', "sin(f(x))");

    println!("sin(1/3) = {}", bank.get(&'s').unwrap().evaluate(&bank, Fraction::from(1)));
}

#[cfg(test)]
mod derivative_tests {
    use super::*;

    #[test]
    fn basic_deriv() {
        let mut bank = ap_calc::new_bank('f',"3x");
        ap_calc::add_func_to_bank(&mut bank, 'd', "f'(x)");
        println!("f'(x) = {}", ap_calc::derivative::ddx(bank.get(&'f').unwrap().get_expression()));
        println!("f'(5) = {}", bank.get(&'d').unwrap().evaluate(&bank, Fraction::from(5)));
    }

    #[test]
    fn x_prime() {
        let mut bank = ap_calc::new_bank('f',"x");
        ap_calc::add_func_to_bank(&mut bank, 'd', "f'(x)");
        println!("f'(x) = {}", ap_calc::derivative::ddx(bank.get(&'f').unwrap().get_expression()));
        println!("f'(5) = {}", bank.get(&'d').unwrap().evaluate(&bank, Fraction::from(5)));
    }

    #[test]
    fn add_deriv() {
        let mut bank = ap_calc::new_bank('f',"3+x");
        ap_calc::add_func_to_bank(&mut bank, 'd', "f'(x)");
        println!("f'(x) = {}", ap_calc::derivative::ddx(bank.get(&'f').unwrap().get_expression()));
        println!("f'(5) = {}", bank.get(&'d').unwrap().evaluate(&bank, Fraction::from(5)));
    }

    #[test]
    fn multiplication_deriv() {
        let mut bank = ap_calc::new_bank('f',"3*x");
        ap_calc::add_func_to_bank(&mut bank, 'd', "f'(x)");
        println!("f'(x) = {}", ap_calc::derivative::ddx(bank.get(&'f').unwrap().get_expression()));
        println!("f'(5) = {}", bank.get(&'d').unwrap().evaluate(&bank, Fraction::from(5)));
    }

    #[test]
    fn x_double_prime() {
        let mut bank = ap_calc::new_bank('f',"x''");
        println!("x'' = {}", bank.get(&'f').unwrap());
        println!("x''@x=5 = {}", bank.get(&'f').unwrap().evaluate(&bank, Fraction::from(5)));
    }

    #[test]
    fn multi_deriv() {
        let mut bank = ap_calc::new_bank('f',"3*x");
        ap_calc::add_func_to_bank(&mut bank, 'a', "f'(x)");
        ap_calc::add_func_to_bank(&mut bank, 'd', "f''(x)");
        println!("f''(x) = {}", ap_calc::derivative::ddx(&ap_calc::derivative::ddx(bank.get(&'f').unwrap().get_expression())));
        println!("f''(5) = {}", bank.get(&'d').unwrap().evaluate(&bank, Fraction::from(5)));
    }
}
