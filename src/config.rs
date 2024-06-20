#![allow(dead_code)]
use abi_stable::std_types::RString;
use anyhow::Result;
use serde::Deserialize;

// Credits: https://github.com/github/gemoji/blob/master/db/emoji.json
const DEFAULT_EMOJIS: &str = include_str!("../res/emoji.json");
const CONFIG_FILE: &str = "emoji.ron";

#[derive(Debug, Clone, Deserialize)]
pub struct Emoji {
    emoji: String,
    description: String,
    category: String,
    aliases: Vec<String>,
    tags: Vec<String>,

    #[serde(skip_deserializing)]
    matches: String,
}
impl AsRef<str> for Emoji {
    fn as_ref(&self) -> &str {
        &self.matches
    }
}

impl Emoji {
    pub fn set_match_items(&mut self) {
        self.matches = format!("{} {} {} {}", self.description, self.category, self.aliases.join(" "), self.tags.join(" "))
    }
    pub fn title(&self) -> &str {
        &self.emoji
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    max_entries: usize,
    prefix: String,
    emoji_path: Option<String>,
    emojis: Vec<Emoji>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_entries: 10,
            prefix: ":".into(),
            emoji_path: None,
            emojis: Vec::new(),
        }
    }
}

impl Config {
    pub fn new(config_dir: RString) -> Self {
        match std::fs::read_to_string(format!("{}/{}", config_dir, CONFIG_FILE)) {
            Ok(content) => ron::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
    }
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn emoji_list(&self) -> Result<Vec<Emoji>> {
        let mut emojis: Vec<Emoji> = serde_json::from_str(&match &self.emoji_path {
            Some(p) => std::fs::read_to_string(p)?,
            None => DEFAULT_EMOJIS.to_string(),
        })?;
        emojis.extend(self.emojis.clone());
        emojis.iter_mut().for_each(|e| e.set_match_items());

        Ok(emojis)
    }
}
