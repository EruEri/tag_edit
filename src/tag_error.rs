use std::io::Error;

/// Representation of the error that may append while handling the tags
#[derive(Debug)]
pub enum TagError {
    IoError(Error),
    ID3TagNotFound,
    ReusedLangDescription,
    LangWrongSize
}