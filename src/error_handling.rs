#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }

    pub fn report(&self, localization: String) {
        eprintln!(
            "[line {} Error {}]: {}",
            self.line, localization, self.message
        )
    }
}
