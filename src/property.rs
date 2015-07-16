/// A `Font` property.
#[derive(PartialEq, Clone, Debug)]
pub enum Property {
	///
	String(String),

	///
	Integer(i64),
}

impl Property {
	/// Parse a property string.
	pub fn parse(string: &str) -> Property {
		if string.starts_with('"') {
			Property::String((&string[1..string.len()]).to_owned())
		}
		else {
			Property::Integer(string.parse().unwrap())
		}
	}
}
