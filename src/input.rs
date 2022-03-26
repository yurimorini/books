extern crate atty;

use crate::books::Isbn;
use crate::cli::Args;
use atty::{is, Stream};
use std::fs::File;
use std::io::{stdin, Read, Result as IoResult};
use thiserror::Error;

/// Delegate to read input
///
pub struct InputReader;

/// Input error to wrap all errors
///
#[derive(Error, Debug)]
pub enum InputError {
    #[error("No input file provided.")]
    NoFileProvided,
    #[error("Cannot read from file \"{0}\": {1}")]
    ReadFileError(String, String),
    #[error("Impossible to read stdin: {0}")]
    IoError(String),
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl InputReader {
    pub fn read(args: &Args) -> Result<Vec<Isbn>, InputError> {
        read_input(args)
    }
}

/// Read the input data from the possible input sources
///
fn read_input(args: &Args) -> Result<Vec<Isbn>, InputError> {
    let has_arguments = !args.isbn_list.is_empty();
    let has_file_option = args.input_file.is_some();

    if !has_arguments {
        if has_file_option {
            let argument = args.input_file.clone();
            let file = argument.ok_or(InputError::NoFileProvided)?;
            let string = read_file(&file)
                .map_err(|e| InputError::ReadFileError(file, e.to_string()))?;
            let input = parse_input_to_isbn(&string);
            return Ok(input);
        } else if has_stream_data() {
            let string =
                read_input_stream().map_err(|e| InputError::IoError(e.to_string()))?;
            let input = parse_input_to_isbn(&string);
            return Ok(input);
        }
    }

    Ok(list_to_isbn(args))
}

/// Convert the arg input strings to a list of Isbn
///
fn list_to_isbn(args: &Args) -> Vec<Isbn> {
    map_list_to_isbn(args.isbn_list.clone())
}

/// Map list of strings to a list of Isbn
///
fn map_list_to_isbn(list: Vec<String>) -> Vec<Isbn> {
    list.iter().map(|s| Isbn::new(s)).collect()
}

/// Map an input string to a list of Isbn
///
fn parse_input_to_isbn(data: &str) -> Vec<Isbn> {
    data.lines()
        .map(|item| item.split(' ').map(|part| Isbn::new(part)))
        .flatten()
        .collect::<Vec<Isbn>>()
}

/// Read a file to string
///
fn read_file(input_file: &str) -> IoResult<String> {
    let mut f = File::open(input_file)?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;
    Ok(buf)
}

/// Returns true if stdin data is set
///
fn has_stream_data() -> bool {
    !is(Stream::Stdin)
}

/// Read the whole stdin in a string
///
fn read_input_stream() -> IoResult<String> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    Ok(input.trim().to_string())
}
