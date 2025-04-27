//! Module dedicated to the [`RemoveFiles`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free coroutine for removing files.
#[derive(Debug)]
pub struct RemoveFiles {
    input: Option<HashSet<PathBuf>>,
}

impl RemoveFiles {
    /// Creates a new coroutine from the given file paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> RemoveFiles {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::RemoveFiles(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::RemoveFiles(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        if let Err(paths) = input {
            return Err(Io::RemoveFiles(Err(paths)));
        };

        Ok(())
    }
}
