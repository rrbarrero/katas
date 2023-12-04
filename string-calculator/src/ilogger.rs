use std::{fs::File, io::Write};

pub trait ILogger {
    fn write(&mut self, data: &str);
}


/* FILE IMPLEMENTATION */

pub struct FileLogger {
    file: File,
}

impl FileLogger {
    pub fn new() -> Self {
        let file = File::create("/tmp/kata.log").expect("Unable to create file");
        Self { file }
    }
}

impl ILogger for FileLogger {
    fn write(&mut self, data: &str) {
        self.file
            .write_all(data.as_bytes())
            .expect("Unable to write file");
    }
}

/* IN MEMORY IMPLEMENTATION */

pub struct InMemoryLogger {
    data: String,
}

impl InMemoryLogger {
    pub fn new() -> Self {
        Self { data: String::new() }
    }
}

impl ILogger for InMemoryLogger {
    fn write(&mut self, data: &str) {
        self.data.push_str(data);
    }
}
