use crate::errors;

pub struct InputsParser {
    numbers: Vec<i64>,
}

impl InputsParser {
    pub fn new(numbers_as_string: &str) -> Result<Self, errors::CustomError> {
        let delimiter = Self::get_delimiter(numbers_as_string);
        let normalized = Self::normalize_string(numbers_as_string, &delimiter);
        let numbers = Self::string_to_numbers_under_1000(&normalized);
        let negative_numbers = Self::extract_negatives(&numbers);
        if !negative_numbers.is_empty() {
            return Err(errors::CustomError { negative_numbers });
        }
        Ok(Self { numbers })
    }

    fn extract_negatives(numbers: &[i64]) -> Vec<i64> {
        numbers
            .iter()
            .filter(|&n| n.is_negative())
            .cloned()
            .collect()
    }

    fn normalize_string(numbers_as_string: &str, delimiter: &str) -> String {
        numbers_as_string.replace('\n', ",").replace(delimiter, ",")
    }

    fn get_delimiter(numbers_as_string: &str) -> String {
        let mut delimiter = String::from(",");
        if numbers_as_string.starts_with("//") {
            delimiter.clear();
            for character in numbers_as_string.chars().skip(2) {
                if character == '\n' {
                    break;
                }
                delimiter.push(character);
            }
        }
        delimiter
    }

    fn string_to_numbers_under_1000(numbers_as_string: &str) -> Vec<i64> {
        numbers_as_string
            .split(',')
            .map(|n| n.parse::<i64>().unwrap_or(0))
            .filter(|&n| n < 1001)
            .collect()
    }

    pub fn get_numbers(&self) -> Vec<i64> {
        self.numbers.clone()
    }
}
