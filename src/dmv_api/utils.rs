use super::structs::{Location, LocationMap};
use std::fs::File;
use std::io::Read;

pub struct Utils {
	location_mapping: LocationMap,
}

impl Utils {
	pub fn new() -> Self {
		let mut file = File::open("licensingLocations.json").unwrap();
		let mut data = String::new();
		file.read_to_string(&mut data).unwrap();
		let mapping: LocationMap = serde_json::from_str(data.as_str()).unwrap();

		Utils {
			location_mapping: mapping,
		}
	}
	pub fn get_location_from_id(&self, id: &str) -> &Location {
		&self.location_mapping[id]
	}
	pub fn get_location_mapping(&self) -> &LocationMap {
		&self.location_mapping
	}
}

#[cfg(test)]
mod tests {
	use crate::structs::Location;
	use crate::utils::Utils;

	#[test]
	fn it_creates_a_utils_struct() {
		Utils::new();
	}

	#[test]
	fn it_returns_the_location_mapping() {
		let utils = Utils::new();
		assert_eq!(&utils.location_mapping, utils.get_location_mapping())
	}

	#[test]
	fn it_returns_the_correct_location() {
		let utils = Utils::new();
		let freehold_location = Location {
			location_id: String::from("197"),
			location_title: String::from("Freehold"),
			location_street: String::from("811 Okerson Road"),
			location_town: String::from("Freehold"),
			location_zip: String::from("07728"),
		};
		assert_eq!(
			utils.get_location_from_id(&freehold_location.location_id.to_string()),
			&freehold_location
		)
	}
}
