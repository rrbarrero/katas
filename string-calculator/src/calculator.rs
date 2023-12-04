use crate::ilogger::ILogger;

pub struct Calculator {
    logger: Box<dyn ILogger>,
}

impl Calculator {
    pub fn new(logger: Box<dyn ILogger>) -> Self {
        Self { logger }
    }

    pub fn sum_numbers(mut self, numbers: Vec<i64>) -> i64 {
        let result = numbers.iter().sum();
        self.logger.write(&format!("{}", result));
        result
    }
}