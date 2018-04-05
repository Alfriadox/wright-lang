//! Module containing struct to represent Interpreter Errors.

use errors::{Error, ErrorLevel};
use position::span::Span;

/// Struct for representing errors generated by the interpreter 
/// when trying to read or create files.
///
#[derive(Debug, Copy, Clone)]
pub struct InterpreterError<'source: 'error, 'error> {
    /// Name of the file involved in the Error.
    pub file_name: &'source str,
    /// A reference to a single element array containing
    /// a string reference ([`&str`])
    ///
    /// # Examples:
    /// ```rust
    /// use wright::interpreter::interpreter_error::InterpreterError;
    /// use wright::errors::Error;
    /// let err = InterpreterError {
    ///     file_name: "example.wr",
    ///     reason: "Could not open file.",
    /// };
    /// err.display();
    /// ```
    /// [`&str`]: https://doc.rust-lang.org/stable/std/str/
    pub reason: &'error str,
}

impl<'src: 'err, 'err> Error<'src, 'err> for InterpreterError<'src, 'err> {
    fn get_name(&self) -> &'err str { "I/O Error" }
    fn get_module(&self) -> &'src str { self.file_name }
    fn get_level(&self) -> ErrorLevel { ErrorLevel::Error }
    fn get_spans(&self) -> Vec<Span> { vec![] }
    fn get_info(&self) -> Vec<&'err str> { vec![self.reason] }
    fn get_lines(&self) -> &'src [&'src str] { &[] }
}