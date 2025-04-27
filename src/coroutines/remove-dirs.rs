//! Module dedicated to the [`RemoveDirs`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free coroutine for removing directories.
#[derive(Debug)]
pub struct RemoveDirs {
    input: Option<HashSet<PathBuf>>,
}

impl RemoveDirs {
    /// Creates a new coroutine from the given directory paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> RemoveDirs {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::RemoveDirs(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::RemoveDirs(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        if let Err(paths) = input {
            return Err(Io::RemoveDirs(Err(paths)));
        };

        Ok(())
    }
}
