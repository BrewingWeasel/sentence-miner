## Unreleased

### Added

### Fixed

- Exporting originally detected lemma even when the users manually specifies another one

## 0.3.6 - 2025-01-26

### Added

### Fixed

- Stanza not being able to correctly identify the lemma of words with diacritics in certain cases
- `&nbsp;` being displayed in the reader
- Fixed a few more cases where the reader would display weird line breaks
- Words directly after punctuation being interpreted as the punctuation
- Selecting context for an export removing any context not selected

## 0.3.5 - 2024-08-26

### Added

- Basic support for epub files

### Fixed

- problems with mwt tokens
- some problems with weird line breaks in the reader (still more to solve though)

## 0.3.4 - 2024-08-25

### Added

- Menu to tell when syncing word knowledge with anki is complete

### Fixed

- Typo in wizard
- Changes made in wizard not always being saved
- Showing word knowledge refresh buttons when they won't work

## 0.3.3 - 2024-08-25

### Added

- Ability to customize ankiconnect port
- German template
- Arabic template
- Docs about when stanza is required

### Fixed

- Changing template's name sometimes causing problems
- RTL languages being displayed LTR

## 0.3.2 - 2024-08-24

### Fixed

- Not being able to open on ARM mac

## 0.3.1 - 2024-08-24

### Added

- Button to uninstall stanza
- Up to date README documentation, photos etc

### Fixed

- Issue with receiving events in the debug version
- Not giving an error when pip install fails
- Issues with running out of space in flatpak (hopefully)
- Scrolling looking ugly on grammar page

## 0.3.0 - 2024-08-23

### Added

- More control with dictionaries and a variety of types
- Better exporting support
- More input types

### Changed

- Completely reworked the UI (switched to vue)
- Switched to Stanza from SpaCy

## [0.1.0] - 2023-10-28

### Added

- Support for custom styles
- Support for any html

### Changed

- Switched to tauri + leptos, so the entire UI is different

## [0.1.0] - 2023-10-08

### Added

- Support for reading text from the clipboard
- Highlighting text based on grammatical properties
- Automatically detecting the lemma of each word
- Settings menu
- Support for multiple dictionaries from a server, file with custom delimiter and stardict
- Using a custom spacy model
