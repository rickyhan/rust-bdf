use std::fmt;
use std::error;
use std::io;
use std::num;

pub enum Error {
	IO(io::Error),
	Parse(num::ParseIntError),

	MissingVersion,
	MissingValue(String),
	Unknown,
	End,

	MalformedFont,
	MalformedProperties,
	MalformedChar,
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::IO(value)
	}
}

impl From<num::ParseIntError> for Error {
	fn from(value: num::ParseIntError) -> Self {
		Error::Parse(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(f.write_str("bdf::Error("));
		try!(fmt::Display::fmt(self, f));
		f.write_str(")")
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			&Error::IO(ref err) =>
				err.description(),

			&Error::Parse(..) =>
				"Parsing error.",

			&Error::MissingVersion =>
				"Missing version from STARTFONT",

			&Error::MissingValue(..) =>
				"Missing value for property.",

			&Error::Unknown =>
				"An unknown entry has been found.",

			&Error::End =>
				"End of file reached.",

			&Error::MalformedFont =>
				"Malformed font definition.",

			&Error::MalformedProperties =>
				"Malformed properties definition.",

			&Error::MalformedChar =>
				"Malformed character definition.",
		}
	}
}