use serde::Deserialize;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
	#[serde(rename = "locationID")]
	pub location_id: String,
	pub location_title: String,
	pub location_street: String,
	pub location_town: String,
	pub location_zip: String,
}

pub type LocationMap = HashMap<String, Location>;
