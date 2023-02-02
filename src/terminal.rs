use dialoguer::{theme::ColorfulTheme, Input};

pub fn term_init() -> u32 {
	let input: u32 = Input::with_theme(&ColorfulTheme::default())
		.with_prompt("How many images to download?")
		.interact_text()
		.unwrap();
	println!("Number of images to download: {}", input);
	return input;
}
