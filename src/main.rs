use std::time::Duration;

use clap::{Arg, Command};

mod retry;
use retry::retry;

fn main() {
	let matches = Command::new("ctry")
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.help_template(
			"{name} {version} - {about}\n\
			{usage-heading} {usage}\n\
			{all-args}{after-help}")
		.before_help(format!("ctry {} - Retry CLI commands", env!("CARGO_PKG_VERSION")))
		.arg(
			Arg::new("max")
				.short('m')
				.long("max")
				.value_name("MAX_RETRIES")
				.help("Maximum retries, use 0 for unlimited retries")
				.num_args(1)
				.default_value("5")
		)
		.arg(
			Arg::new("interval")
				.short('i')
				.long("interval")
				.value_name("INTERVAL")
				.help("Interval in seconds between the retries")
				.num_args(1)
				.default_value("1")
		)
		.arg(
			Arg::new("exit_code")
				.short('e')
				.long("exitcode")
				.value_name("EXIT_CODE")
				.help("On which exit code retries will stop")
				.num_args(1)
				.default_value("0")
		)
		.arg(
			Arg::new("quiet")
				.short('q')
				.long("quiet")
				.help("Suppress output of the command")
				.action(clap::ArgAction::SetTrue)
		)
		.trailing_var_arg(true)
		.arg(
			Arg::new("command")
				.value_name("COMMAND")
				.help("Command to run")
				.required(true)
				.num_args(1..)
		)
		.get_matches();

	let max_retries: u32 = matches
		.get_one::<String>("max")
		.unwrap()
		.parse()
		.expect("The given MAX option must be an Integer");
	let interval = Duration::from_secs(
		matches
			.get_one::<String>("interval")
			.unwrap()
			.parse()
			.expect("The given INTERVAL option must be an Integer"),
	);
	let exitcode = matches
		.get_one::<String>("exit_code")
		.unwrap()
		.parse()
		.expect("The given exit code option must be an Integer");
	let quiet = matches.get_flag("quiet");
	let cmd: Vec<&str> = matches
		.get_many::<String>("command")
		.unwrap()
		.map(String::as_str)
		.collect();
	let config = retry::RetryConfig {
		max: max_retries,
		interval,
		expected_exitcode: exitcode,
		quiet,
		cmd,
	};
	retry(config);
}
