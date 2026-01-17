// Instructor version:
// fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
//     let mut found = false;

//     // Cycle through the languages until you land on a match, and return
//     // the one right after the match if there is one...
//     for lang in languages {
//         if found {
//             return lang.as_str();
//         }

//         if lang == current {
//             found = true;
//         }
//     }

//     // If no match is found or nothing comes after the match, just return
//     // something, or in this case, just return the very last language available
//     languages.last().unwrap().as_str()
// }

// Requires lifetime annotation because we have two ref inputs and 
// don't know what one is getting returned
pub fn next_language<'a>(languages: &'a [String], current: &str) -> Option<&'a str> {
    languages
        .iter()
        .position(|s| s == current)
        .and_then(|i| languages.get(i+1))
        .map(|s| s.as_str())
}

// Does not require lifetime annotation because we only have one 
// input reference and can safely assume it's the only one being returned
// either fully or as a slice in this case (points back to the original).
// We can call this use of ommission of the lifetime annotation being "elided" 
// or just "elision". Nevermind the pendantic use of the word "elision"...  
pub fn last_language(languages: &[String]) -> &str {
    let last = languages
        .iter()
        .last();

    match last {
        Some(s) => s.as_str(),
        None => "NOTHING"
    }
    // "NOTHING" // living in .rodata somewhere
}

// Instructor version:
// fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
//      if lang_a.len() >= lang_b.len() {
//          lang_a
//      } else {
//          lang_b
//      } 
// }
pub fn longest_language(languages: &[String]) -> &str {
    let longest = languages
        .iter()
        .max_by_key(|s| s.len())
        .map(|s| s);
    
    match longest {
        Some(s) => s.as_str(),
        None => "NOTHING",
    }
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

    #[test]
    fn test_longest_language() {
        let languages = vec![
            String::from("rust"),
            String::from("go"),
            String::from("typescript"),
        ];

        let longest_lang = longest_language(&languages);
        let expected = "typescript";
        
        assert_eq!(longest_lang, expected);
    }

}