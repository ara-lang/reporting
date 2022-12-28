use codespan_reporting::files::Error as CodespanError;

/// An enum representing an error that happened while looking up a file or a piece of content in that file.
#[derive(Debug)]
pub enum Error {
    /// A required file is not in the file database.
    FileMissing,
    /// The file is present, but does not contain the specified byte index.
    IndexTooLarge { given: usize, max: usize },
    /// The file is present, but does not contain the specified line index.
    LineTooLarge { given: usize, max: usize },
    /// The file is present and contains the specified line index, but the line does not contain the specified column index.
    ColumnTooLarge { given: usize, max: usize },
    /// The given index is contained in the file, but is not a boundary of a UTF-8 code point.
    InvalidCharBoundary { given: usize },
    /// There was a error while doing IO.
    Io(std::io::Error),
    /// Codespan error.
    CodespanError(CodespanError),
}
