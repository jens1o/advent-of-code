use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    iter,
    sync::atomic::{AtomicU16, Ordering},
    time::Instant,
};

mod permutations;
#[cfg(test)]
mod tests;

type NumberType = u64;

#[derive(Debug, PartialEq, PartialOrd)]
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

fn get_all_permutations(number_len: usize) -> impl Iterator<Item = Vec<Operator>> {
    let set = iter::repeat_n(Operator::ADD, number_len)
        .chain(iter::repeat_n(Operator::MULTIPLY, number_len))
        // .chain(iter::repeat_n(Operator::CONCATENATE, number_len))
        ;

    permutations::UniquePermutations::new(set.collect())
}

fn apply_operators(equation: &Equation, operators: &Vec<Operator>) -> bool {
    let Equation {
        numbers,
        test_value,
    } = equation;

    debug_assert!(!numbers.is_empty());
    debug_assert_eq!(numbers.len(), operators.len() + 1);

    let mut result = numbers[0];

    for (next_number, operator) in numbers[1..].iter().zip(operators.iter()) {
        match operator {
            Operator::ADD => result += next_number,
            Operator::MULTIPLY => result *= next_number,
            Operator::CONCATENATE => {
                result = format!("{result}{next_number}")
                    .parse::<NumberType>()
                    .unwrap()
            }
        }

        if &result > test_value {
            return false;
        }
    }

    &result == test_value
}

fn is_valid_equation(equation: &Equation) -> bool {
    let instant = Instant::now();

    let needed_iterator_length = equation.numbers.len() - 1;

    let permutations = get_all_permutations(needed_iterator_length);

    for permutation in permutations {
        let result = apply_operators(&equation, &permutation);

        if result {
            // yada yada yada
            dbg!(instant.elapsed());
            return true;
        }
    }

    dbg!(instant.elapsed());

    false
}

fn get_sum_of_valid_equations(equation_list: Vec<Equation>) -> NumberType {
    let processed_equations = AtomicU16::new(0);

    equation_list
        .par_iter()
        .map(|equation| {
            let result = is_valid_equation(equation);
            dbg!(processed_equations.fetch_add(1, Ordering::SeqCst));

            if result {
                equation.test_value
            } else {
                0
            }
        })
        .sum()
}
