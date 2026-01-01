
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    // Cycle through the languages until you land on a match, and return
    // the one right after the match if there is one...
    for lang in languages {
        if found {
            return lang.as_str();
        }

        if lang == current {
            found = true;
        }
    }

    // If no match is found or nothing comes after the match, just return
    // something, or in this case, just return the very last language available
    languages.last().unwrap().as_str()
}

// Requires lifetime annotation because we have two ref inputs and 
// don't know what one is getting returned
fn next_language_v2<'a>(languages: &'a [String], current: &str) -> Option<&'a str> {
    languages
        .iter()
        .position(|s| s == current)
        .and_then(|i| languages.get(i+1))
        .map(|s| s.as_str())
}

// Does not require lifetime annotation because we only have one 
// input reference and can safely assume it's the only one being returned
// either fully or as a slice in this case (points back to the oringal)
fn last_language(languages: &[String]) -> &str {
    let last = languages
        .iter()
        .last();

    match last {
        Some(s) => s.as_str(),
        None => "NOTHING"
    }
    // "NOTHING" // living in .rodata somewhere
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_language() {
        let languages = vec![
            String::from("rust"),
            String::from("go"),
            String::from("typescript"),
        ];

        // try next langauge 
        let cur_lang = next_language_v2(&languages, "rust");
        let expected = "go";

        assert_eq!(cur_lang.unwrap(), expected);
    }

    #[test]
    fn test_last_language() {
        let languages = vec![
            String::from("rust"),
            String::from("go"),
            String::from("typescript"),
        ];

        let last_lang = last_language(&languages);
        let expected = "typescript";

        assert_eq!(last_lang, expected);

    }
}

fn main() {
    println!("Starting language handler...");

    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let mut language = match languages.get(0) {
        Some(s) => s,
        None => "NONE",
    };

    println!("Current language: {:#?}", language);
    language = next_language(&languages, &language); 
    println!("Next language: {:#?}", language);

    println!("Current language: {:#?}", language);
    language = match next_language_v2(&languages, &language) {
        Some(s) => s,
        None => "NONE",
    };
    println!("Next language: {:#?}", language);

    println!("Last language: {:#?}", last_language(&languages)); 
}
