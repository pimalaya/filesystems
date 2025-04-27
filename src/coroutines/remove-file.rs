//! Module dedicated to the [`RemoveFile`] I/O-free coroutine.

use std::path::PathBuf;

use crate::Io;

/// I/O-free coroutine for removing a file.
#[derive(Debug)]
pub struct RemoveFile {
    input: Option<PathBuf>,
}

impl RemoveFile {
    /// Creates a new coroutine from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::RemoveFile(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::RemoveFile(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        if let Err(path) = input {
            return Err(Io::RemoveFile(Err(path)));
        };

        Ok(())
    }
}
