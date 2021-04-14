use super::utils::Utils;
use reqwest::Url;
use std::collections::HashMap;

// TODO: Add traits & impl. to enums to allow for a standardized function that returns the necessary query pairs
enum DmvEndpoints {
	GetNextAvailableDate,
	AppointmentWizzard,
}

struct Client {
	utils: Utils,
}

impl Client {
	pub fn new() -> Self {
		Client {
			utils: Utils::new(),
		}
	}

	#[inline]
	pub fn url(&self) -> &'static str {
		"https://telegov.njportal.com/njmvc/"
	}
	async fn get_request(&self, endpoint: &str) -> reqwest::Result<HashMap<String, String>> {
		let resp = reqwest::get(endpoint)
			.await?
			.json::<HashMap<String, String>>()
			.await;
		resp
	}
	fn build_endpoint(&self, endpoint: DmvEndpoints, location_id: &str) -> String {
		match endpoint {
			DmvEndpoints::GetNextAvailableDate => {
				let mut result = Url::parse(self.url())
					.unwrap()
					.join("CustomerCreateAppointments/GetNextAvailableDate")
					.unwrap();
				result
					.query_pairs_mut()
					.append_pair("appointmentTypeId", "15")
					.append_pair("locationId", location_id);
				result.as_str().to_string()
			}
			DmvEndpoints::AppointmentWizzard => Url::parse(self.url())
				.unwrap()
				.join("AppointmentWizard/15/")
				.unwrap()
				.join(location_id)
				.unwrap()
				.as_str()
				.to_string(),
			_ => self.url().to_string(),
		}
	}
	async fn get_next_available_appointment(
		&self,
		location_id: &str,
	) -> reqwest::Result<HashMap<String, String>> {
		let result = self
			.get_request(&self.build_endpoint(DmvEndpoints::GetNextAvailableDate, location_id))
			.await;
		result
	}
	async fn check_available_appointments(&self) {
		self.utils.get_location_mapping();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn make_a_new_client() {
		Client::new();
	}

	#[test]
	fn it_builds_next_available_appointment_endpoint() {
		let comparison_endpoint = "https://telegov.njportal.com/njmvc/CustomerCreateAppointments/GetNextAvailableDate?appointmentTypeId=15&locationId=197";
		let client = Client::new();
		assert_eq!(
			comparison_endpoint,
			client.build_endpoint(DmvEndpoints::GetNextAvailableDate, "197")
		)
	}

	#[tokio::test]
	async fn it_fires_off_get_request() {
		let client = Client::new();
		let result = client.get_next_available_appointment("197").await.unwrap();
	}
}
