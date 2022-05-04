use std::path::Path;

use rand::Rng;

pub struct Config {
    pub span: bool,
    pub keyword: String,
}

pub fn parse_cli_args(args: Vec<String>) -> Config {
    let span_string = "span".to_string();
    let span = args.contains(&span_string);

    let mut keywords = args.to_vec();
    remove_element(&mut keywords, span_string);

    let keyword = if keywords.is_empty() {
        "wallpaper".to_string()
    } else {
        choose_random_keyword(keywords)
    };

    Config { span, keyword }
}

fn choose_random_keyword(keywords: Vec<String>) -> String {
    return if keywords.len() > 1 {
        let random_index = rand::thread_rng().gen_range(0..keywords.len());
        keywords.get(random_index).unwrap().to_string()
    } else {
        keywords.get(0).unwrap().to_string()
    };
}

fn remove_element(keywords: &mut Vec<String>, term: String) {
    let index = keywords.iter().position(|item| *item == term);

    if let Some(index) = index {
        keywords.remove(index);
    }
}

pub fn is_url(keyword: &str) -> bool {
    keyword.starts_with("http") && keyword.contains("://")
}

pub fn is_local_path(keyword: &String) -> bool {
    return Path::new(keyword).exists();
}
