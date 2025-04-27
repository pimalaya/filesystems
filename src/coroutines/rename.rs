//! Module dedicated to the [`Rename`] I/O-free coroutine.

use std::path::PathBuf;

use crate::Io;

/// I/O-free coroutine for renaming files or directories.
#[derive(Debug)]
pub struct Rename {
    input: Option<Vec<(PathBuf, PathBuf)>>,
}

impl Rename {
    /// Reads a new coroutine from the given source and destination paths.
    pub fn new(
        sources: impl IntoIterator<Item = (impl Into<PathBuf>, impl Into<PathBuf>)>,
    ) -> Self {
        let sources = sources
            .into_iter()
            .map(|(from, to)| (from.into(), to.into()))
            .collect();

        Self {
            input: Some(sources),
        }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(paths) => Io::Rename(Err(paths)),
                None => Io::UnavailableInput,
            });
        };

        let Io::Rename(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        if let Err(paths) = input {
            return Err(Io::Rename(Err(paths)));
        }

        Ok(())
    }
}
