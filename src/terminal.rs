use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

pub fn server_or_dl() -> bool {
	let mut sod: bool = false;
	let choices = vec!["Server Mode", "Download Images"];
	let selection = Select::with_theme(&ColorfulTheme::default())
		.items(&choices)
		.default(0)
		.interact_on_opt(&Term::stderr())
		.unwrap();
	match selection {
		Some(0) => sod = true,
		Some(1) => sod = false,
		None => println!("User did not choose"),
		_ => (),
	}
	sod
}

pub fn term_init() -> u32 {
	let input: u32 = Input::with_theme(&ColorfulTheme::default())
		.with_prompt("How many images to download?")
		.interact_text()
		.unwrap();
	return input;
}
