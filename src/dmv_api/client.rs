use super::utils::Utils;
use reqwest::Url;
use std::collections::HashMap;
use super::sms::SMS;
use std::fmt::Error;
use crate::structs::SchedulerError;

// TODO: Add traits & impl. to enums to allow for a standardized function that returns the necessary query pairs
enum DmvEndpoints {
	GetNextAvailableDate,
	AppointmentWizzard,
}

struct Client {
	utils: Utils,
	sms: SMS
}

impl Client {
	pub fn new() -> Self {
		Client {
			utils: Utils::new(),
			sms: SMS::new()
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
	async fn check_appointment_response(&self, response_result: reqwest::Result<HashMap<String, String>>) -> Result<bool, reqwest::Error> {
		match response_result {
			Ok(response) => {
				if response.contains_key("next") && response["next"] != "No Appointments Available" {
					return Ok(true)
				}
				Ok(false)
			}
			Err(e) => Err(e)
		}
	}
	async fn check_available_appointments(&self) -> Result<bool, SchedulerError> {
		let locations = self.utils.get_location_id_collection();
		for location_id in locations {
			let response_result = self.get_next_available_appointment(&location_id).await;
			match self.check_appointment_response(response_result).await {
				Ok(appointment_available) => {
					if appointment_available {
						self.sms.alert_receipients(self.utils.get_location_from_id(&location_id), &self.build_endpoint(DmvEndpoints::AppointmentWizzard, &location_id)).await?
						Ok(true)
					}
				}
				Err(e) => return Err(SchedulerError::SmsError(e))
			}
		}
		Ok(false)
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
		println!("{:#?}", result)
	}

	#[tokio::test]
	async fn it_checks_the_response() {
		let client = Client::new();
		let response = client.get_next_available_appointment("197").await;
		let result = client.check_appointment_response(response).await;
		print!("{}", result)
	}
}
