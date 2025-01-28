use regex::Regex;

#[derive(Debug)]
pub struct MulInstruction(u64, u64);

impl MulInstruction {
    pub fn calc_product(&self) -> u64 {
        self.0 * self.1
    }
}

#[allow(unused)]
pub fn third_december() {
    let mut instructions_enabled = true;
    let mut last_instruction_pos = None;

    let instructions_keywords = Regex::new(r#"do\(\)|don't\(\)"#).unwrap();

    let mut picked_up_instructions: Vec<MulInstruction> = Vec::new();

    let payload = include_str!("part3-input.txt");
    for keyword_match in instructions_keywords.find_iter(payload) {
        if instructions_enabled {
            let current_section = match last_instruction_pos {
                Some(pos) => &payload[pos..keyword_match.start()],
                None => &payload[0..keyword_match.start()],
            };
            dbg!(&current_section);
            picked_up_instructions.extend(get_uncorrupted_mul_instructions(current_section));
        }

        instructions_enabled = keyword_match.as_str() == "do()";
        last_instruction_pos = Some(keyword_match.end());
    }

    // get the very last part if we're still active at the end
    if instructions_enabled {
        if let Some(last_instruction_pos) = last_instruction_pos {
            picked_up_instructions.extend(get_uncorrupted_mul_instructions(
                &payload[last_instruction_pos..],
            ));
        }
    }

    let sum_of_products = picked_up_instructions
        .iter()
        .fold(0, |accumulator, current| {
            current.calc_product() + accumulator
        });

    dbg!(sum_of_products);
}

pub fn get_uncorrupted_mul_instructions(input_string: impl AsRef<str>) -> Vec<MulInstruction> {
    let mut found_instructions = Vec::new();
    let re = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();

    for (_, [first, second]) in re.captures_iter(input_string.as_ref()).map(|c| c.extract()) {
        found_instructions.push(MulInstruction(
            first.parse().unwrap(),
            second.parse().unwrap(),
        ));
    }

    found_instructions
}

#[cfg(test)]
mod tests {
    use super::get_uncorrupted_mul_instructions;

    #[test]
    fn given_example() {
        let payload = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let uncorrupted_instructions = get_uncorrupted_mul_instructions(payload);

        let sum_of_products = uncorrupted_instructions
            .iter()
            .fold(0, |accumulator, current| {
                current.calc_product() + accumulator
            });

        assert_eq!(sum_of_products, 161);
    }
}
