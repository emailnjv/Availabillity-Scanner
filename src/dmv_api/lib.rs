use log::{debug, error, info, trace, warn};
use fern::colors::{Color, ColoredLevelConfig};
use std::fmt::Error;

mod client;
mod sms;
mod structs;
mod utils;


/// This module contains of all the functions & currently just exposes a single run function.
pub async fn run() -> Result<(), Error> {
	let client = client::Client::new();
	setup_logger().expect("couldn't setup the logger");
	client.run().await;
	Ok(())
}

fn setup_logger() -> Result<(), fern::InitError> {
	// configure colors for the whole line
	let colors_line = ColoredLevelConfig::new()
			.error(Color::Red)
			.warn(Color::Yellow)
			// we actually don't need to specify the color for debug and info, they are white by default
			.info(Color::Green)
			.debug(Color::Green)
			// depending on the terminals color scheme, this is the same as the background color
			.trace(Color::BrightBlack);

	// configure colors for the name of the level.
	// since almost all of them are the same as the color for the whole line, we
	// just clone `colors_line` and overwrite our changes
	let colors_level = colors_line.clone().info(Color::Green);

	fern::Dispatch::new()
			.format(move |out, message, record| {
				out.finish(format_args!(
					"{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
					color_line = format_args!(
						"\x1B[{}m",
						colors_line.get_color(&record.level()).to_fg_str()
					),
					date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
					target = record.target(),
					level = colors_level.color(record.level()),
					message = message,
				));
			})
			// set the default log level. to filter out verbose log messages from dependencies, set
			// this to Warn and overwrite the log level for your crate.
			// .level(log::LevelFilter::Warn)
			// change log levels for individual modules. Note: This looks for the record's target
			// field which defaults to the module path but can be overwritten with the `target`
			// parameter:
			// `info!(target="special_target", "This log message is about special_target");`
			.level_for("dmv_api", log::LevelFilter::Trace)
			// output to stdout
			.chain(std::io::stdout())
			.apply()
			.unwrap();

	Ok(())
}