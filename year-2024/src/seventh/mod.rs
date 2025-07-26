use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{
    iter,
    sync::atomic::{AtomicU16, Ordering},
    time::Instant,
};

mod permutations;
#[cfg(test)]
mod tests;

type NumberType = u64;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Equation {
    test_value: NumberType,
    numbers: Vec<NumberType>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum Operator {
    ADD,
    MULTIPLY,
    CONCATENATE,
}

pub(crate) fn seventh_december() {
    let equations = parse_equation_list(include_str!("part7-input.txt"));

    dbg!(get_sum_of_valid_equations(equations));
}

fn parse_equation_list(input: impl AsRef<str>) -> Vec<Equation> {
    let instant = Instant::now();

    let input = input.as_ref().trim();
    let mut equations = Vec::new();

    for line in input.lines() {
        let mut components = line.split(' ');

        let test_value = components
            .next()
            .and_then(|x| x.strip_suffix(':'))
            .and_then(|x| x.parse::<NumberType>().ok())
            .unwrap();

        equations.push(Equation {
            test_value,
            numbers: components
                .map(|x| x.parse::<NumberType>().unwrap())
                .collect(),
        });
    }

    dbg!(instant.elapsed());

    equations
}

fn apply_operators(equation: Equation, mut last_result: NumberType, operator: Operator) -> bool {
    let Equation {
        numbers,
        test_value,
    } = equation;

    debug_assert!(!numbers.is_empty());

    let Some((next_number, numbers)) = numbers.split_first() else {
        panic!();
    };

    match operator {
        Operator::ADD => last_result += next_number,
        Operator::MULTIPLY => last_result *= next_number,
        Operator::CONCATENATE => {
            last_result = format!("{last_result}{next_number}")
                .parse::<NumberType>()
                .unwrap()
        }
    }

    if last_result > test_value {
        return false;
    }

    if !numbers.is_empty() {
        apply_operators(
            Equation {
                test_value,
                numbers: numbers.to_vec(),
            },
            last_result,
            Operator::ADD,
        ) || apply_operators(
            Equation {
                test_value,
                numbers: numbers.to_vec(),
            },
            last_result,
            Operator::MULTIPLY,
        ) || apply_operators(
            Equation {
                test_value,
                numbers: numbers.to_vec(),
            },
            last_result,
            Operator::CONCATENATE,
        )
    } else {
        last_result == test_value
    }
}

fn is_valid_equation(equation: Equation) -> bool {
    let instant = Instant::now();

    let result = apply_operators(equation.clone(), 0, Operator::ADD)
        || apply_operators(equation.clone(), 0, Operator::MULTIPLY)
        || apply_operators(equation.clone(), 0, Operator::CONCATENATE);

    dbg!(instant.elapsed());

    result
}

fn get_sum_of_valid_equations(equation_list: Vec<Equation>) -> NumberType {
    let processed_equations = AtomicU16::new(0);

    equation_list
        .into_par_iter()
        .map(|equation| {
            let test_value = equation.test_value;

            let result = is_valid_equation(equation);
            dbg!(processed_equations.fetch_add(1, Ordering::SeqCst));

            if result {
                test_value
            } else {
                0
            }
        })
        .sum()
}
