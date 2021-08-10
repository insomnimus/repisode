use clap::{
	crate_version,
	App,
	AppSettings,
	Arg,
};

pub fn new() -> App<'static> {
	let app = App::new("repisode")
		.about("Fix episode numbers to sort properly.")
		.version(crate_version!())
		.setting(AppSettings::UnifiedHelpMessage)
		.setting(AppSettings::ArgRequiredElseHelp);

	let interactive = Arg::new("interactive")
		.about("Prompt for confirmation before renaming a file.")
		.short('i')
		.long("interactive");

	let pattern = Arg::new("pattern")
		.about("The pattern that will be used to capture episode numbers.")
		.long_about(
			"The pattern that will be used to capture episode numbers.
Example:
	'Rick And Morty Season 1 Episode @N@ - *.mp4'
Just replace the current season number with `@N@`
`@N@` must match any number of digits.
The `*` wildcard can be used to match anything.",
		)
		.required(true)
		.validator(|s| {
			if !s.contains("@N@") {
				Err(format!(
					"{}: the pattern must contain at least one `@N@` for the fix to work",
					s
				))
			} else {
				Ok(())
			}
		});

	let file = Arg::new("file")
		.about("Any number of files to fix.")
		.required(true)
		.multiple(true);

	let force = Arg::new("force")
		.short('f')
		.long("force")
		.about("Do not prompt when a rename may overwrite a file.");

	let err_abort = Arg::new("err-abort")
		.short('e')
		.long("err-abort")
		.about("Stop any remaining operation in case of an error.");

	app.arg(interactive)
		.arg(force)
		.arg(err_abort)
		.arg(pattern)
		.arg(file)
}
