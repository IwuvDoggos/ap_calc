use super::*;

pub fn ddx(expression: &Expression) -> Expression {
    if let Expression::Equa(equation) = expression {
        let fs = format!("{}",equation.element1);
        let gs = format!("{}",equation.element2);
        let f = Expression::from(&fs);
        let g = Expression::from(&gs);
        let fp = ddx(&f);
        let gp = ddx(&g);
        //println!("====g'&f' test======\nf' = {fp}\ng' = {gp}\n====================");

        let s = match &equation.operation {
            Operation::Add =>
                format!("({fp})+({gp})"),
            Operation::Sub =>
                format!("({fp})-({gp})"),
            Operation::Mult =>
               format!("(({g})({fp}))+(({f})({gp}))"),
            Operation::Div =>
                format!("((({g})({fp}))-(({f})({gp})))/(({g})^2)"),
            Operation::Func =>
                format!("({fp}({g}))({gp})"),
            Operation::Exp =>
               format!("(({f})^({g}))((({gp})(ln({f})))+((({fp})({g}))/({f})))"),
            Operation::Trig =>
                if let Expression::Variable(name) = f {
                    match name {
                        's' => format!("((cos({g}))({gp})))"),
                        'c' => format!("(0-(sin({g})))({gp})))"),
                        't' => format!("((1/(cos({g})^2))({gp})))"),
                        'S' => format!("((1/((1-({g})^2))))^(1/2))({gp})))"),
                        'T' => format!("((1/(1+(({g})^2)))({gp})))"),
                        _ => panic!("wrong!!!!"),
                    }
                } else {
                    panic!("EMERGENCY EMERGENCY EMERGENCY")
                },
            Operation::Log => 
                match f {
                    Expression::Variable('e') => format!("(1/({g}))({gp}))"),
                    Expression::Variable(a) => format!("(1/(({g})(ln({a})))({gp}))"),
                    Expression::Constant(a) => format!("(1/(({g})(ln({a})))({gp}))"),
                    _ => panic!("OOPS, broken in log of ddx"),
                },
            Operation::Deriv => 
                format!("{f}''"),
            _ => panic!("Operation not covered by ddx in match expression"),
        };
        Expression::from(&s)
    } else{
        let es = format!("{}",expression);
        let e  = Expression::from(&es);
        Expression::Equa(Box::new(Equation {
            operation: Operation::Deriv,
            element1: Box::new(e),
            element2: Box::new(Expression::Variable('?')),
        }))
    }
}

pub fn eval_deriv(exp: Expression, bank: &Bank, x: Fraction) -> Expression {
    // exp = element1  of prev what you came from
   
    let exp_cpy = copy_expression(&exp);

    // if constant = 0
    // if x = 1
    // if non x variable, eval_deriv of .get()
    // if deriv function, recursion eval_Deriv of element1
    // if non deriv function, ddx of itself
    match exp {
        Expression::Constant(_c)  => Expression::from("0"),
        Expression::Variable('x') => Expression::from("1"),
        Expression::Variable(f)   => ddx(&get_expression(bank, &exp_cpy)),
        Expression::Equa(e)       => {
            let e1_cpy = copy_expression(&e.element1);
            if e.operation == Operation::Deriv {
                eval_deriv(eval_deriv(e1_cpy,bank,x),bank,x)
            } else {
                ddx(&exp_cpy) 
            }
        },
        _ => panic!("its in the match of the eval_deriv")
    }
}

