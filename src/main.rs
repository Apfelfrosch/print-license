const MIT_TEXT: &str = include_str!("license_texts/TEXT_MIT");
const APACHE_2_0_TEXT: &str = include_str!("license_texts/TEXT_APACHE");

const STORED_TEXTS: [(&str, &str); 2] = [("mit", MIT_TEXT), ("apache-2.0", APACHE_2_0_TEXT)];

fn stored_license_text_names() -> Box<[&'static str]> {
    let res: Vec<&str> = STORED_TEXTS.iter().map(|(name, _)| *name).collect();
    res.into_boxed_slice()
}

fn exit_error() -> ! {
    let mut names_formatted = String::new();

    names_formatted.push('[');
    let names = stored_license_text_names();
    for (idx, name) in names.iter().enumerate() {
        names_formatted.push_str(name);
        if idx < names.len() - 1 {
            names_formatted.push_str(", ");
        }
    }
    names_formatted.push(']');

    eprintln!("Error: Provide a license name {}", names_formatted);
    std::process::exit(1)
}

fn main() {
    let which = std::env::args().nth(1);
    if let Some(which) = which {
        let to_print = STORED_TEXTS.iter().find_map(|(name, text)| {
            if name == &which.as_str() {
                Some(*text)
            } else {
                None
            }
        });
        if let Some(license_text) = to_print {
            print!("{license_text}");
            return;
        }
    }

    exit_error();
}
