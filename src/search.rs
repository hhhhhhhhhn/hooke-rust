use crate::Text;
use websearch;
use regex::Regex;
use reqwest;
use tokio;

pub fn get_queries(text: Text) -> Vec<String>  {
    let words: Vec<&str> = text.iter().map(|unit| unit.value.as_str()).collect();
    return words.chunks(32).map(|words| words.join(" ")).collect()
}

pub fn extract_text(html: String) -> String {
    let style = Regex::new(r"<style([\s\S]*?)</style>").unwrap();
    let script = Regex::new(r"<script([\s\S]*?)</script>").unwrap();
    let tags = Regex::new(r"<[^>]+>").unwrap();
    let empty = Regex::new(r"\n\s*\n").unwrap();

    let html = style.replace_all(&html, "");
    let html = script.replace_all(&html, "");
    let html = tags.replace_all(&html, "");
    return empty.replace_all(&html, "\n").to_string();
}

pub async fn search_similar_texts(text: Text) -> Vec<(String, String)> {
    let mut urls = Vec::new();
    for query in get_queries(text) {
        for url in websearch::searx(&query).await.unwrap_or_else(|| vec!()) {
            if !urls.contains(&url) {
                urls.push(url)
            }
        }
    }
    let texts = download_texts(&urls).await;
    return urls.iter().map(|x| x.clone()).zip(texts).collect();
}

async fn download_texts(urls: &Vec<String>) -> Vec<String> {
    let tasks = urls.iter().map(|x| x.clone())
        .map(|url| {
            tokio::spawn(async move {
                match reqwest::get(&url).await {
                    Ok(request) => {
                        match request.text().await {
                            Ok(text) => {
                                Some(extract_text(text))
                            }
                            Err(_) => None
                        }
                    }
                    Err(_) => None
                }
            })
        });
    
    let mut texts = Vec::new();
    for task in tasks {
        match task.await {
            Ok(Some(text)) => texts.push(text),
            _ => texts.push("".to_string()),
        }
    }
    return texts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::english;

    #[test]
    fn test_extract_html() {
        let text = extract_text("<script>no</script><p><a>yes</a></p>".to_string());
        assert_eq!(text, "yes".to_string());
    }

    #[tokio::test]
    async fn test_search_similar_texts() {
        let text = english::split("In information theory, linguistics, and computer science, the Levenshtein distance is a string metric for measuring the difference between two sequences");
        for int in search_similar_texts(text).await {
            println!("{}\n\n", int.1)
        }
    }
}
