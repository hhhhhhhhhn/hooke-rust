use crate::{Text, Unit};

pub fn shingle(text: Text, k: usize) -> Text {
    let mut new_text = Vec::new();
    for i in 0..(text.len() - k + 1) {
        let value: Vec<String> = text.get(i..(i+k)).unwrap()
            .iter().map(|x| x.value.clone()).collect();
        new_text.push(Unit {
            value: value.join(" "),
            position: (
                text.get(i).unwrap().position.0,
                text.get(i+k-1).unwrap().position.1
            ),
            index: i,
        })
    }
    return new_text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::english;

    #[test]
    fn test_shingle() {
        let words = english::split("one two three four five");
        let shingled = shingle(words, 3);

        assert_eq!(
            shingled.iter().map(|x| x.value.as_str()).collect::<Vec<&str>>(),
            vec!("one two three", "two three four", "three four five")
        )
    }
}
