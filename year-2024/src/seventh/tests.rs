use crate::seventh::{
    apply_operators, get_all_permutations, get_sum_of_valid_equations, is_valid_equation,
    parse_equation_list, Equation, Operator,
};

#[test]
fn test_sample() {
    const SAMPLE: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let equations = parse_equation_list(SAMPLE);

    assert_eq!(
        equations,
        [
            Equation {
                test_value: 190,
                numbers: vec![10, 19]
            },
            Equation {
                test_value: 3267,
                numbers: vec![81, 40, 27]
            },
            Equation {
                test_value: 83,
                numbers: vec![17, 5]
            },
            Equation {
                test_value: 156,
                numbers: vec![15, 6]
            },
            Equation {
                test_value: 7290,
                numbers: vec![6, 8, 6, 15]
            },
            Equation {
                test_value: 161011,
                numbers: vec![16, 10, 13]
            },
            Equation {
                test_value: 192,
                numbers: vec![17, 8, 14]
            },
            Equation {
                test_value: 21037,
                numbers: vec![9, 7, 18, 13]
            },
            Equation {
                test_value: 292,
                numbers: vec![11, 6, 16, 20]
            }
        ]
    )
}

#[test]
fn test_permutations() {
    let permutations = get_all_permutations(1).collect::<Vec<_>>();

    dbg!(&permutations);

    use Operator::*;
    assert_eq!(
        permutations,
        [
            [ADD, ADD],
            [ADD, MULTIPLY],
            [MULTIPLY, ADD],
            [MULTIPLY, MULTIPLY],
        ]
    );
}

#[test]
fn test_apply_operators() {
    assert_eq!(
        apply_operators(
            &Equation {
                test_value: 190,
                numbers: vec![10, 19]
            },
            &vec![Operator::MULTIPLY]
        ),
        true
    );
}

#[test]
fn test_is_valid_equation() {
    let input = Equation {
        test_value: 190,
        numbers: vec![10, 19],
    };

    assert!(is_valid_equation(&input));
}

#[test]
fn test_sum_of_valid_equations() {
    let equation_list = vec![
        Equation {
            test_value: 190,
            numbers: vec![10, 19],
        },
        Equation {
            test_value: 3267,
            numbers: vec![81, 40, 27],
        },
        Equation {
            test_value: 83,
            numbers: vec![17, 5],
        },
        Equation {
            test_value: 156,
            numbers: vec![15, 6],
        },
        Equation {
            test_value: 7290,
            numbers: vec![6, 8, 6, 15],
        },
        Equation {
            test_value: 161011,
            numbers: vec![16, 10, 13],
        },
        Equation {
            test_value: 192,
            numbers: vec![17, 8, 14],
        },
        Equation {
            test_value: 21037,
            numbers: vec![9, 7, 18, 13],
        },
        Equation {
            test_value: 292,
            numbers: vec![11, 6, 16, 20],
        },
    ];

    assert_eq!(11387, get_sum_of_valid_equations(equation_list));
}
