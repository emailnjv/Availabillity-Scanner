use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SchedulerError {
	SmsError(reqwest::Error),
}
impl fmt::Display for SchedulerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			SchedulerError::SmsError(e) => write!(f, "Sms reqwest error: {}", e),
		}
	}
}
impl Error for SchedulerError {}

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
