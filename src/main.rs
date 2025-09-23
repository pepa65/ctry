mod retry;

use std::time::Duration;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

use retry::retry;

fn main() {
	let matches = App::new("ctry")
		.version(crate_version!())
		.about(crate_description!())
		.arg(
			Arg::with_name("max")
				.short("m")
				.long("max")
				.value_name("MAX_RETRIES")
				.help("Maximum retries, use 0 for unlimited retries")
				.takes_value(true)
				.default_value("5"),
		)
		.arg(
			Arg::with_name("interval")
				.short("i")
				.long("interval")
				.value_name("INTERVAL")
				.help("Interval in seconds between the retries")
				.takes_value(true)
				.default_value("1"),
		)
		.arg(
			Arg::with_name("exit_code")
				.short("e")
				.long("exitcode")
				.value_name("EXIT_CODE")
				.help("On which exit code retries will stop")
				.takes_value(true)
				.default_value("0"),
		)
		.arg(Arg::with_name("quiet").short("q").long("quiet").help("Suppress output of the command"))
		.setting(AppSettings::TrailingVarArg)
		.arg(Arg::with_name("command").value_name("COMMAND").help("Command to run").required(true).multiple(true).last(true))
		.get_matches();
	let max_retries: u32 = match matches.value_of("max").unwrap_or_default().parse() {
		Ok(u) => u,
		Err(_) => panic!("The given MAX option must be an Integer"),
	};
	let interval = Duration::from_secs(match matches.value_of("interval").unwrap_or_default().parse() {
		Ok(u) => u,
		Err(_) => panic!("The given INTERVAL option must be an Integer"),
	});
	let exitcode = match matches.value_of("exit_code").unwrap_or_default().parse() {
		Ok(c) => c,
		Err(_) => panic!("The given exit code option must be an Integer"),
	};
	let quiet = matches.is_present("quiet");
	let cmd: Vec<&str> = matches.values_of("command").unwrap().collect();
	let config = retry::RetryConfig { max: max_retries, interval, expected_exitcode: exitcode, quiet, cmd };
	retry(config);
}
