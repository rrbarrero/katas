use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CustomError{
    pub negative_numbers: Vec<i64>,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Negative numbers not allowed!")?;
        let negative_numbers_str: String = self.negative_numbers
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}", negative_numbers_str)
    }
}

impl Error for CustomError {}