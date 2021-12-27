//! Representation of the error that may append while handling the tags

use std::io::Error;

#[derive(Debug)]
pub enum TagError {
    IoError(Error),
    ID3TagNotFound,
    ReusedLangDescription,
    LangWrongSize
}