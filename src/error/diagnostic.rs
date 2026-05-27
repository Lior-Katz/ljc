use crate::file::Span;
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug)]
pub struct Diagnostic<T: Display> {
    pub span: Span,
    pub message: T,
}

pub trait Diagnose
where
    Self: Sized + Display,
{
    fn at(self, span: Span) -> Diagnostic<Self>;
}

impl<T> Diagnose for T
where
    T: Display,
{
    fn at(self, span: Span) -> Diagnostic<Self> {
        Diagnostic { span, message: self }
    }
}

#[derive(Debug)]
pub struct SourceWithDiagnostic<'a, T: Display> {
    file: &'a Path,
    source: &'a str,
    error: Diagnostic<T>,
}

impl<'a, T: Display> SourceWithDiagnostic<'a, T> {
    pub fn new(file_name: &'a Path, source: &'a str, error: Diagnostic<T>) -> Self {
        Self { file: file_name, source, error }
    }
}

impl<'a, T: Display> Display for SourceWithDiagnostic<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.error.message)?;
        if let Some(file_name) = self.file.to_str() {
            write!(f, "{}:", file_name)?;
        }
        writeln!(f, "{}", self.error.span)?;
        if let Some(line) = self.error.span.source_line(self.source) {
            writeln!(f, "{line}")?;
            writeln!(f, "{}^", " ".repeat(self.error.span.column - 1))
        } else {
            Ok(())
        }
    }
}
