//! Module dedicated to the [`ReadDir`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free coroutine for reading directory entries.
#[derive(Debug)]
pub struct ReadDir {
    input: Option<PathBuf>,
}

impl ReadDir {
    /// Reads a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<HashSet<PathBuf>, Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::ReadDir(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::ReadDir(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(contents) => Ok(contents),
            Err(path) => Err(Io::ReadDir(Err(path))),
        }
    }
}
