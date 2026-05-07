use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_template() -> Option<String> {
    let templates = vec!["Express"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the project template")
        .items(&templates)
        .default(0)
        .interact()
        .unwrap();

    Some(templates[selection].to_lowercase())
}
