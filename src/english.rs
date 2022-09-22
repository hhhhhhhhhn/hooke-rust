extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};
use regex::Regex;
use stop_words;
use crate::{Text, Unit};

pub fn split(s: &str) -> Text {
    let s = s.to_lowercase();
    let re = Regex::new(r"[a-zA-Z]+").unwrap();

    return re.find_iter(&s)
        .enumerate()
        .map(|(i, m)|
             Unit {
                 value: m.as_str().to_string(),
                 position: (m.start(), m.end()),
                 index: i
             }
        ).collect()
}

pub fn remove_stopwords(text: Text) -> Text {
    return text.iter()
        .filter(|word| !stop_words::get("en").contains(&word.value))
        .enumerate()
        .map(|(i, word)| 
             Unit{
                 value: word.value.clone(),
                 position: word.position,
                 index: i
             }
        )
        .collect()
}

pub fn stem(text: Text) -> Text {
    let stemmer = Stemmer::create(Algorithm::English);
    return text.iter()
        .map(|word| 
             Unit{
                 value: stemmer.stem(&word.value).to_string(),
                 index: word.index,
                 position: word.position
             }
        )
        .collect()
}

// Does splitting, removing stopword, and stemming
pub fn preprocess(s: &str) -> Text {
    return stem(remove_stopwords(split(s)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let words = split("These are some normal (?) English words.");
        assert_eq!(
            words.iter().map(|x| x.value.as_str()).collect::<Vec<&str>>(),
            vec!("these", "are", "some", "normal", "english", "words")
        )
    }

    #[test]
    fn test_remove_stopwords() {
        let words = split("Some words are completely unnecessary, like is");
        let words = remove_stopwords(words);
        assert_eq!(
            words.iter().map(|x| x.value.as_str()).collect::<Vec<&str>>(),
            vec!("completely", "unnecessary")
        )
    }

    #[test]
    fn test_stem() {
        let words = split("Some words are completely unnecessary, like 'is'");
        let words = remove_stopwords(words);
        let words = stem(words);
        assert_eq!(
            words.iter().map(|x| x.value.as_str()).collect::<Vec<&str>>(),
            vec!("complet", "unnecessari")
        )
    }
}
