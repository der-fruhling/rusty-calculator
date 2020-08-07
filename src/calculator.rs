extern crate assert_approx_eq;

use crate::tokenizer::tokenize;
use std::str::FromStr;
use std::f64::consts::{PI, E};
#[allow(unused_imports)] // Unused normally, perfectly ok.
use assert_approx_eq::assert_approx_eq;
use std::collections::HashMap;

//static VARIABLES: HashMap<String, f64> = HashMap::new();

fn consume(tokens: &mut Vec<String>, variables: &mut HashMap<String, f64>) -> f64 {
    let token = &*tokens.remove(0);
    match token {
        "+" => consume(tokens, variables) + consume(tokens, variables),
        "-" => consume(tokens, variables) - consume(tokens, variables),
        "*" => consume(tokens, variables) * consume(tokens, variables),
        "/" => consume(tokens, variables) / consume(tokens, variables),
        "=" => {
            let name = tokens.remove(0);
            let c = consume(tokens, variables);
            variables.insert(name, c);
            c
        },
        "print" => {
            let value = consume(tokens, variables);
            println!("=> {}", value);
            value
        }
        "sin" => consume(tokens, variables).sin(),
        "cos" => consume(tokens, variables).cos(),
        "pi" => PI,
        "e" => E,
        _ => {
            if variables.contains_key(token) {
                *variables.get(token).unwrap()
            } else {
                f64::from_str(token).unwrap()
            }
        },
    }
}

pub(crate) fn calculate(expression: &str, variables: &mut HashMap<String, f64>) -> f64 {
    consume(&mut Vec::from(&*tokenize(expression)), variables)
}

#[test]
fn test_calculate() {
    let variables: &mut HashMap<String, f64> = &mut HashMap::new();
    assert_eq!(calculate("+ 10 5", variables), 15.0);
    assert_eq!(calculate("- 10 5", variables), 5.0);
    assert_eq!(calculate("* 10 5", variables), 50.0);
    assert_eq!(calculate("/ 10 5", variables), 2.0);
    assert_approx_eq!(calculate("sin pi", variables), 0.0, 0.01f64);
    assert_approx_eq!(calculate("cos pi", variables), -1.0, 0.01f64);
    assert_eq!(calculate("pi", variables), PI);
    assert_eq!(calculate("e", variables), E);
    assert_eq!(calculate("+ + 2 3 4", variables), 9.0);
    assert_eq!(calculate("+ + 2 3 - 4 3", variables), 6.0);
    assert_eq!(calculate("= $a 3", variables), 3.0);
    assert_eq!(calculate("print $a", variables), 3.0);
}