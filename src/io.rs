use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

/// The filesystems I/O request enum, emitted by [coroutines] and
/// processed by [runtimes].
///
/// Represents all the possible I/O requests that a filesystem
/// coroutine can emit. Runtimes should be able to handle all
/// variants.
///
/// [coroutines]: crate::coroutines
/// [runtimes]: crate::runtimes
#[derive(Debug)]
pub enum Io {
    UnavailableInput,
    UnexpectedInput(Box<Io>),

    CreateDir(Result<(), PathBuf>),
    CreateDirs(Result<(), HashSet<PathBuf>>),
    CreateFile(Result<(), (PathBuf, Vec<u8>)>),
    CreateFiles(Result<(), HashMap<PathBuf, Vec<u8>>>),
    ReadDir(Result<HashSet<PathBuf>, PathBuf>),
    ReadFile(Result<Vec<u8>, PathBuf>),
    ReadFiles(Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>),
    RemoveDir(Result<(), PathBuf>),
    RemoveDirs(Result<(), HashSet<PathBuf>>),
    RemoveFile(Result<(), PathBuf>),
    RemoveFiles(Result<(), HashSet<PathBuf>>),
    Rename(Result<(), Vec<(PathBuf, PathBuf)>>),
}
