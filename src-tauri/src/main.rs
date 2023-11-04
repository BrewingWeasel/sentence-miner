// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{add_to_anki::add_to_anki, dictionary::get_def, language_parsing::parse_text};
use ankiconnect::get_anki_card_statuses;
use shared::Settings;
use std::{collections::HashMap, fs, sync::Mutex};
use tauri::{async_runtime::block_on, State};

mod add_to_anki;
mod ankiconnect;
mod dictionary;
mod language_parsing;

struct SakinyjeState(Mutex<SharedInfo>);

struct SharedInfo {
    words: HashMap<String, WordInfo>,
    cached_defs: HashMap<String, Vec<String>>,
    settings: Settings,
}

impl Default for SharedInfo {
    fn default() -> Self {
        let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");

        let settings: Settings = fs::read_to_string(config_file)
            .map(|v| toml::from_str(&v).unwrap()) // TODO: some sort of error handling when invalid
            // toml is used
            .unwrap_or_default();

        let mut words = HashMap::new();

        if let Some(ankiparsers) = &settings.anki_parser {
            for (deck, note_parser) in ankiparsers {
                block_on(get_anki_card_statuses(&deck, note_parser, &mut words)).unwrap();
                // TODO: handle error
            }
        }
        Self {
            words,
            cached_defs: HashMap::new(),
            settings,
        }
    }
}

struct WordInfo {
    rating: u8,
    can_modify: bool,
}

fn main() {
    tauri::Builder::default()
        .manage(SakinyjeState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            parse_text,
            get_def,
            get_settings,
            add_to_anki,
            write_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_settings(state: State<SakinyjeState>) -> Settings {
    let state = state.0.lock().unwrap();
    state.settings.clone()
}

#[tauri::command]
fn write_settings(state: State<SakinyjeState>, settings: Settings) -> Result<(), String> {
    let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
    let conts = toml::to_string_pretty(&settings).unwrap();

    let mut state = state.0.lock().unwrap();
    state.settings = settings;

    fs::write(config_file, conts).map_err(|e| e.to_string())
}
