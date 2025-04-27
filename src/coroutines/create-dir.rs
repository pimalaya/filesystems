//! Module dedicated to the [`CreateDir`] I/O-free coroutine.

use std::path::PathBuf;

use crate::Io;

/// I/O-free coroutine for creating a directory.
#[derive(Debug)]
pub struct CreateDir {
    input: Option<PathBuf>,
}

impl CreateDir {
    /// Creates a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::CreateDir(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::CreateDir(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        if let Err(path) = input {
            return Err(Io::CreateDir(Err(path)));
        };

        Ok(())
    }
}
