use scraper::{Html, Selector};
use serde::Deserialize;

use crate::puzzle::Puzzle;

#[derive(Debug)]
pub struct Error(String);

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self(format!("{}", e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self(format!("{}", e))
    }
}

#[derive(Deserialize, Debug)]
struct NytimesPuzzleData {
    puzzle: Vec<u8>,
}

#[derive(Deserialize, Debug)]
struct NytimesLevel {
    puzzle_data: NytimesPuzzleData
}

#[derive(Deserialize, Debug)]
struct NytimesPuzzles {
    easy: NytimesLevel,
    medium: NytimesLevel,
    hard: NytimesLevel,
}

impl NytimesPuzzles {
    fn easy(self) -> Result<Puzzle, Error> { Self::puzzle_from(self.easy.puzzle_data.puzzle) }
    fn medium(self) -> Result<Puzzle, Error> { Self::puzzle_from(self.medium.puzzle_data.puzzle) }
    fn hard(self) -> Result<Puzzle, Error> { Self::puzzle_from(self.hard.puzzle_data.puzzle) }

    fn puzzle_from(input: Vec<u8>) -> Result<Puzzle, Error> {
        if input.len() != 81 {
            return Err(Error("Unexpected number of cells in NYTimes data.".to_string()));
        }

        let mut cells: [u8; 81] = [0; 81];

        for (idx, value) in input.iter().enumerate() {
            cells[idx] = *value;
        }

        Ok(Puzzle::from(cells))
    }
}

fn fetch_nytimes() -> Result<NytimesPuzzles, Error> {
    let resp = reqwest::blocking::get("https://www.nytimes.com/puzzles/sudoku/medium")?;

    let doc = Html::parse_document(resp.text()?.as_str());

    let selector = Selector::parse("#sudoku-container script").unwrap();

    // The script contains a JSON object prefixed with 'window.gameData = ', as of 2021-01-02.
    let script = doc.select(&selector).nth(0)
        .ok_or(Error("Could not find sudoku script code in response.".to_string()))?
        .inner_html()
        .replace("window.gameData = ", "");

    let result = serde_json::from_str(script.as_str())?;

    Ok(result)
}

pub fn fetch(downloader: &str) -> Result<Puzzle, Error> {
    match downloader {
        "nytimes-easy" => fetch_nytimes()?.easy(),
        "nytimes-medium" => fetch_nytimes()?.medium(),
        "nytimes-hard" => fetch_nytimes()?.hard(),
        _ => Err(Error("Unknown download source.".to_string())),
    }
}
