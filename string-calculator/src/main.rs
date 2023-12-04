mod calculator;
mod errors;
mod ilogger;
mod inputs;

use std::{
    env,
    io::{self, Write},
};

use calculator::Calculator;
use ilogger::{InMemoryLogger, ILogger};
use inputs::InputsParser;

pub fn sum(strings_numbers: &str, logger: Option<Box<dyn ILogger>>) -> i64 {
    let inputs = InputsParser::new(strings_numbers).unwrap();

    let logger = logger.unwrap_or_else(|| Box::new(InMemoryLogger::new()));

    let calculator = Calculator::new(logger);
    calculator.sum_numbers(inputs.get_numbers())
}


fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn dispatcher() {
    let mut another_one = "Yes".to_string();

    while !another_one.is_empty() {
        let numbers_input = get_user_input("Enter a list of numbers separated by commas: ");
        let result = sum(numbers_input.as_str(), None);
        println!("The result is {}", result);

        another_one = get_user_input("another input please: ");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(numbers_as_string) = args.get(1) {
        println!("The result is {}", sum(numbers_as_string, None));
    }
    dispatcher()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{ilogger::FileLogger, sum, Calculator, InputsParser};

    #[test]
    fn test_string_calculator_sum_0() {
        assert_eq!(sum("", None), 0);
    }

    #[test]
    fn test_string_calculator_sum_one_number() {
        assert_eq!(sum("7", None), 7);
    }

    #[test]
    fn test_string_calculator_sum_two_numbers() {
        assert_eq!(sum("1,2", None), 3);
    }

    #[test]
    fn test_string_calculator_sum_three_numbers() {
        assert_eq!(sum("6,6,6", None), 18);
    }

    #[test]
    fn test_string_calculator_new_lines_as_separator_chart() {
        assert_eq!(sum("1\n2,3", None), 6);
    }

    #[test]
    fn test_string_calculator_change_delimiter_chart() {
        assert_eq!(sum("//;\n1;2", None), 3);
    }

    #[test]
    fn test_string_calculator_negative_number_raises_error() {
        let _ = InputsParser::new("//;\n1;2;-3;5;-1").is_err();
    }

    #[test]
    fn test_string_calculator_numbers_above_1000_are_ignored() {
        assert_eq!(sum("1001,2", None), 2);
    }

    #[test]
    fn test_string_calculator_numbers_aunder_1000_are_summed() {
        assert_eq!(sum("1000,2", None), 1002);
    }

    #[test]
    fn test_string_calculator_logging_data() {
        let inputs = InputsParser::new("1000,2").unwrap();
        let calculator = Calculator::new(Box::new(FileLogger::new()));
        assert_eq!(calculator.sum_numbers(inputs.get_numbers()), 1002);
        let file_content = fs::read_to_string("/tmp/kata.log").unwrap();
        assert_eq!(file_content, "1002");
    }
}
