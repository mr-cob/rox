pub struct Error {
    line: usize,
    message: String,
}

impl Error {
    pub fn new(line: usize, message: &str) -> Self {
        Error {
            line,
            message: message.to_string(),
        }
    }
}

impl Error {
    pub fn print(&self) {
        println!("Error: {}, line {}.", self.message, self.line);
    }
}
