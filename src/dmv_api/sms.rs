use std::env::var;

use crate::structs::{Location, SchedulerError};

use twrs_sms::TwilioSend;

/// The struct containing the data & implementations for interacting with the TwilioAPI
pub struct SMS {
	target_numbers: Vec<String>,
	source_number: String,
	account_sid: String,
	account_token: String,
}

impl<'a> SMS {
	/// Returns SMS API struct
	///
	/// Implementation of this struct requires the following ENV variables to be set:
	///  - TW_TO="COUNTRYCODE_PHONENUMBER,COUNTRYCODE_PHONENUMBER,..."
	///  - TW_FROM="COUNTRYCODE_PHONENUMBER"
	///  - TW_SID="ACCOUNT_SID"
	///  - TW_TOKEN="ACCOUNT_TOKEN"
	pub fn new() -> Self {
		SMS {
			target_numbers: var("TW_TO")
				.unwrap()
				.split(',')
				.map(|number| number.to_owned())
				.collect(),
			source_number: var("TW_FROM").unwrap(),
			account_sid: var("TW_SID").unwrap(),
			account_token: var("TW_TOKEN").unwrap(),
		}
	}

	/// Takes in the information that's needed to create a message to send to the end recipient, returning the String msg.
	///
	/// Currently takes in the location title & the URL to schedule the appointment, formatting it into a String including various line breaks.
	fn create_message_body(&self, location_title: &str, appointment_url: &str) -> String {
		format!(
			r"Appointment Available!
Location: {}

Schedule your appointment at this url: {}

Reply STOP to unsubscribe",
			location_title, appointment_url
		)
	}

	/// Takes in the resulting message & returns a vector of ind. TwilioSend structs to be sent
	///
	/// This function iterates over the self.target_numbers property to determine how many send structs are needed.
	fn create_messages(&'a self, message_body: &'a str) -> Vec<TwilioSend> {
		// let message_body = self.create_message_body(&location.location_title, appointment_url);
		let mut result = Vec::new();
		for number in self.target_numbers.iter() {
			result.push(TwilioSend {
				Body: &message_body,
				From: &self.source_number,
				To: number,
			})
		}
		result
	}

	// TODO: sort out lifetimes
	/// Iterates over the vector of TwilioSend structs sending them to Twilio
	async fn send_messages<'b>(
		&self,
		twilio_sends: Vec<TwilioSend<'b>>,
	) -> Result<(), SchedulerError> {
		for message in twilio_sends {
			let encoded_msg = message
				.encode()
				.expect("Error converting to url encoded string");
			let response = twrs_sms::send_message(&self.account_sid, &self.account_token, encoded_msg)
				.await
				.expect("Error with HTTP request");
			// Run the loop to make sure the message was delivered
			twrs_sms::is_delivered(response, &self.account_sid, &self.account_token)
				.await
				.expect("Error SMS not delivered");
		}
		Ok(())
	}

	/// This is the main exposed function of the struct allowing the main client to fire off the messages when an appointment is found.
	pub async fn alert_receipients(
		&self,
		location: &Location,
		appointment_url: &str,
	) -> Result<(), SchedulerError> {
		let msg_body = self.create_message_body(&location.location_title, appointment_url);
		let twilio_sends = self.create_messages(&msg_body);
		self.send_messages(twilio_sends).await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::Utils;
	use std::env;

	#[test]
	fn it_creates_an_sms_client() {
		SMS::new();
	}

	#[test]
	fn it_parses_env_variables() {
		env::set_var("TW_TO", "1_8488888888,1_8489999999,1_8480000000");
		env::set_var("TW_FROM", "1_8481111111");
		env::set_var("TW_SID", "00000000000000000000000000000");
		env::set_var("TW_TOKEN", "aaaaaaaaaaaaaaaaaaaaaaaaaaaa");

		let expected = vec!["1_8488888888", "1_8489999999", "1_8480000000"];

		let sms_client = SMS::new();
		assert_eq!(sms_client.target_numbers, expected)
	}

	#[test]
	fn it_creates_twilio_sends_for_all_numbers() {
		env::set_var("TW_TO", "1_8488888888,1_8489999999,1_8480000000");
		env::set_var("TW_FROM", "1_8481111111");
		env::set_var("TW_SID", "00000000000000000000000000000");
		env::set_var("TW_TOKEN", "aaaaaaaaaaaaaaaaaaaaaaaaaaaa");

		let sms_client = SMS::new();
		let utils = Utils::new();
		let test_location = utils.get_location_from_id("197");
		let apt_url = String::from("https://telegov.njportal.com/njmvc/AppointmentWizard/15/")
			+ &test_location.location_id;
		let msg_body = sms_client.create_message_body(&test_location.location_title, &apt_url);

		let result = sms_client.create_messages(&msg_body);
		assert_eq!("1_8488888888", result[0].To);
		assert_eq!("1_8489999999", result[1].To);
		assert_eq!("1_8480000000", result[2].To);
	}

	#[tokio::test]
	#[ignore]
	async fn test_full() {
		// Be sure to have the follow environment variables set before running this ignored test
		// export TW_TO="COUNTRYCODE_PHONENUMBER,COUNTRYCODE_PHONENUMBER"
		// export TW_FROM="COUNTRYCODE_PHONENUMBER"
		// export TW_SID="ACCOUNT_SID"
		// export TW_TOKEN="ACCOUNT_TOKEN"

		let sms_client = SMS::new();
		let utils = Utils::new();
		let test_location = utils.get_location_from_id("197");
		let apt_url = String::from("https://telegov.njportal.com/njmvc/AppointmentWizard/15/")
			+ &test_location.location_id;

		sms_client
			.alert_receipients(test_location, &apt_url)
			.await
			.unwrap()
	}
}
