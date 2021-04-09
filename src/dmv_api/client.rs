use super::utils::Utils;
use reqwest::{Response, get, Url};
use std::collections::HashMap;

struct Client {
	utils: Utils,
	url: Url
}

// TODO: Add traits & impl. to enums to allow for a standardized function that returns the necessary query pairs
#[derive(strum_macros::Display)]
enum DmvEndpoints {
	GetNextAvailableDate,
	NavigateToDateTime
}

impl Client {
	pub fn new() -> Self {
		Client {
			utils: Utils::new(),
			url: Url::parse("https://telegov.njportal.com/njmvc/CustomerCreateAppointments").unwrap()
		}
	}

	async fn get(endpoint: &str) -> Result<Response, E> {
		let resp = reqwest::get(endpoint)
				.await?
				.json::<HashMap<String, String>>()
				.await?;
	}

	fn build_endpoint(&self, endpoint: DmvEndpoints, location_id: &str) -> &str {
		// TODO: handle next endpoint given it's not a standard endpoint
		// https://telegov.njportal.com/njmvc/CustomerCreateAppointments/GetNextAvailableDate?appointmentTypeId=15&locationId=186
		self.url.join(endpoint.into()).unwrap()
				.query_pairs_mut()
				.append_pair("appointmentTypeId", "15")
				.append_pair("locationId", location_id);
	}

	async fn get_next_available_appointment(&self, location_id: &str) -> HashMap<String, String> {
		get()
	}

}




#[cfg(test)]
mod tests {
    use super::*;


}
