use super::sms::SMS;
use super::utils::Utils;
use crate::structs::SchedulerError;
use reqwest::Url;
use std::collections::HashMap;

use tokio::time::{sleep, Duration};

// TODO: Add traits & impl. to enums to allow for a standardized function that returns the necessary query pairs
// TODO: Error consolidation & lifetime alignment
// TODO: Consolidate/standardize timer initialization
enum DmvEndpoints {
	GetNextAvailableDate,
	AppointmentWizard,
}

pub struct Client {
	utils: Utils,
	sms: SMS,
}

impl Client {
	pub fn new() -> Self {
		Client {
			utils: Utils::new(),
			sms: SMS::new(),
		}
	}
	pub async fn run(&self) {
		let mut appointment_found: bool = false;
		let mut iteration_counter: i64 = 0;
		while !appointment_found {
			appointment_found = match self.check_available_appointments().await {
				Ok(appointment_found) => appointment_found,
				Err(e) => std::panic::panic_any(e),
			};
			println!("Iteration #{:} finished", iteration_counter);
			iteration_counter += 1;
			// sleep(Duration::from_secs(3)).await;
			// sleep(Duration::from_secs(300)).await;
		}
	}

	#[inline]
	fn url(&self) -> &'static str {
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
			DmvEndpoints::AppointmentWizard => Url::parse(self.url())
				.unwrap()
				.join("AppointmentWizard/15/")
				.unwrap()
				.join(location_id)
				.unwrap()
				.as_str()
				.to_string(),
		}
	}
	async fn get_next_available_appointment(
		&self,
		location_id: &str,
	) -> reqwest::Result<HashMap<String, String>> {
		let endpoint = self.build_endpoint(DmvEndpoints::GetNextAvailableDate, location_id);
		let result = self
			.get_request(&endpoint)
			.await;
		result
	}
	async fn check_appointment_response(
		&self,
		response_result: reqwest::Result<HashMap<String, String>>,
	) -> Result<bool, reqwest::Error> {
		match response_result {
			Ok(response) => {
				if response.contains_key("next") && response["next"] != "No Appointments Available" {
					println!("{:?}", response);
					return Ok(true);
				}
				Ok(false)
			}
			Err(e) => Err(e),
		}
	}
	async fn check_available_appointments(&self) -> Result<bool, SchedulerError> {
		let locations = self.utils.get_location_id_collection();
		for location_id in locations {
			sleep(Duration::from_secs(2)).await;
			let response_result = self.get_next_available_appointment(&location_id).await;
			match self.check_appointment_response(response_result).await {
				Ok(appointment_available) => {
					if appointment_available {
						self
							.sms
							.alert_receipients(
								self.utils.get_location_from_id(&location_id),
								&self.build_endpoint(DmvEndpoints::AppointmentWizard, &location_id),
							)
							.await?;
						return Ok(true);
					}
				}
				Err(e) => return Err(SchedulerError::SmsError(e)),
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
		let mut test_result: HashMap<String, String> = HashMap::new();
		test_result.insert(String::from("next"), String::from("Something Different"));

		let client = Client::new();
		let response = client.get_next_available_appointment("197").await;
		let actual_result = client.check_appointment_response(response).await.unwrap();
		let false_positive_result = client
			.check_appointment_response(Ok(test_result))
			.await
			.unwrap();
		assert_eq!(false, actual_result);
		assert_eq!(true, false_positive_result)
	}

	#[tokio::test]
	#[ignore]
	async fn it_checks_all_locations() {
		let client = Client::new();
		let result = client.check_available_appointments().await.unwrap();
		assert_eq!(false, result)
	}
}
