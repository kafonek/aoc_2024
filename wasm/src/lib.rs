use aoc_2024::day01::Day01;
use aoc_2024::day02::Day02;
use aoc_2024::utils::FromLines;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmDay01(Day01);

#[wasm_bindgen]
pub struct WasmDay02(Day02);

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

#[wasm_bindgen]
impl WasmDay02 {
    #[wasm_bindgen]
    pub fn from_content(content: &str) -> WasmDay02 {
        let data = Day02::from_lines(content.lines().map(String::from));
        WasmDay02(data)
    }

    #[wasm_bindgen]
    pub fn count_safe_reports(&self) -> i32 {
        self.0.count_safe_reports()
    }

    #[wasm_bindgen]
    pub fn count_safe_reports_with_dampener(&self) -> i32 {
        self.0.count_safe_reports_with_dampener()
    }
}
