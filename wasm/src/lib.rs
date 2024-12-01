use aoc_2024::day01::Day01;
use aoc_2024::utils::FromLines;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmDay01(Day01);

#[wasm_bindgen]
impl WasmDay01 {
    #[wasm_bindgen]
    pub fn from_content(content: &str) -> WasmDay01 {
        let data = Day01::from_lines(content.lines().map(String::from));
        WasmDay01(data)
    }

    #[wasm_bindgen]
    pub fn distance(&self) -> i32 {
        self.0.distance()
    }

    #[wasm_bindgen]
    pub fn similarity(&self) -> i32 {
        self.0.similarity()
    }
}
