mod part1_Input;

mod day1 {
    use crate::part1_Input;

    pub fn part_1(_input: &str) -> u64 {
        let mut result = Vec::new();

        let mut result_sum = 0;

        let mut line_count = 0;
        for line in part1_Input::PART1_INPUT.lines() {
            let line_str = line.to_string();

            line_count = line_count + 1;

            let mut char_count = 0;
            let mut first_num: Option<char> = None;
            let mut last_num: Option<char> = None;
            for character in line_str.chars() {
                char_count = char_count + 1;


                if character.is_digit(10) {

                    if first_num.is_none() {
                        first_num = Some(character);
                    }

                    last_num = Some(character);

                }

            }
     
            if first_num.is_some() {
                let mut result_string = String::from(first_num.unwrap());
                if last_num.is_some() {
                    result_string.push(last_num.unwrap());
                }

                result_sum = result_sum + result_string.parse::<u64>().unwrap();

                result.push(result_string);
            }

        }

        println!("Lines: {}", line_count);
        // println!("{:?}", result);
        println!("Result sum: {}", result_sum);
        64
        // part1_Input::PART1_INPUT;
    }
}

aoc_main::main! {
    year 2023;
    day1 => part_1;
}
