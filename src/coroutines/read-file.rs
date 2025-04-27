//! Module dedicated to the [`ReadFile`] I/O-free coroutine.

use std::path::PathBuf;

use crate::Io;

/// I/O-free coroutine for reading file contents.
#[derive(Debug)]
pub struct ReadFile {
    input: Option<PathBuf>,
}

impl ReadFile {
    /// Reads a new coroutine from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<Vec<u8>, Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::ReadFile(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::ReadFile(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(contents) => Ok(contents),
            Err(path) => Err(Io::ReadFile(Err(path))),
        }
    }
}
