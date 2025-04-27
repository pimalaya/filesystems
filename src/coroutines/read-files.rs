//! Module dedicated to the [`ReadFiles`] I/O-free coroutine.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::Io;

/// I/O-free coroutine for reading files contents.
#[derive(Debug)]
pub struct ReadFiles {
    input: Option<HashSet<PathBuf>>,
}

impl ReadFiles {
    /// Reads a new coroutine from the given files path.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<HashMap<PathBuf, Vec<u8>>, Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(paths) => Io::ReadFiles(Err(paths)),
                None => Io::UnavailableInput,
            });
        };

        let Io::ReadFiles(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(contents) => Ok(contents),
            Err(paths) => Err(Io::ReadFiles(Err(paths))),
        }
    }
}
