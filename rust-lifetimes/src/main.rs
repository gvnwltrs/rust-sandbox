
fn next_language<'a>(languages: &'a [String], current: &str) -> Option<&'a str> {
    let mut found = false;

    for lang in languages {
        if found {
            return Some(lang.as_str());
        }

        if lang == current {
            found = true;
        }
    }

    Some(languages.last().unwrap().as_str())
}

fn next_language_v2<'a>(languages: &'a [String], current: &str) -> Option<&'a str> {
    languages
        .iter()
        .position(|s| s == current)
        .and_then(|i| languages.get(i+1))
        .map(|s| s.as_str())
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
        let cur_lang = next_language(&languages, "rust");
        let expected = "go";

        assert_eq!(cur_lang.unwrap(), expected);
    }
}

fn main() {
    println!("Starting language handler...");

    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    println!("Current language: {:#?}",languages.get(0).unwrap());
    let language = next_language(&languages, languages.get(0).unwrap());
    println!("Next language: {:#?}", language.unwrap());

    println!("Current language: {:#?}", language.unwrap());
    let language = next_language_v2(&languages, language.unwrap());
    println!("Next language: {:#?}", language.unwrap());
}
