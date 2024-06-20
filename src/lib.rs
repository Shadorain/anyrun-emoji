use std::process::Command;

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;

mod utils;

mod config;
use config::{Config, Emoji};

pub struct State {
    config: Config,
    emojis: Vec<Emoji>,
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Emojis".into(),
        icon: "accessories-character-map".into(), // Icon from the icon theme
    }
}

#[init]
fn init(config_dir: RString) -> State {
    let config = Config::new(config_dir);
    let emojis = config.emoji_list().expect("Failed to read emoji list");
    State { config, emojis }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    // Return out if the prefix is not present
    let input = match input.strip_prefix(state.config.prefix()) {
        Some(input) => input.trim(),
        None => return RVec::new(),
    };

    // The logic to get matches from the input text in the `input` argument.
    // The `data` is a mutable reference to the shared data type later specified.
    let mut entries = utils::fuzzy_match(input, &state.emojis);
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries.truncate(state.config.max_entries());
    entries
        .into_iter()
        .map(|(item, _)| {
            Match {
                title: item.title().into(),
                description: ROption::RSome(item.description().into()),
                use_pango: false,
                icon: ROption::RNone,
                id: ROption::RNone,
            }
        })
        .collect()
}

/// Handles the selected match
/// Uses `wl-copy` to copy the selected match.
///
/// * `selection`: Selected match
///
/// # Errors
///
/// This function returns how anyrun should proceed
#[handler]
fn handler(selection: Match) -> HandleResult {
    Command::new("wl-copy")
        .arg(selection.title.as_str())
        .spawn()
        .expect("Failed to spawn wl-copy");

    HandleResult::Close
}
