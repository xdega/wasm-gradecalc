extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
fn sort_desc(grades_json: &String) -> Result<String, serde_json::Error> {
    let mut grades: Vec<u64> = serde_json::from_str(&grades_json).unwrap();

    grades.sort_by(|a, b| b.cmp(a));

    serde_json::to_string(&grades)
}

#[wasm_bindgen]
fn sort_asc(grades_json: &String) -> Result<String, serde_json::Error> {
    let mut grades: Vec<u64> = serde_json::from_str(&grades_json).unwrap();

    grades.sort_by(|a, b| a.cmp(b));

    serde_json::to_string(&grades)
}

#[wasm_bindgen]
fn percent(num: i32, max: i32) -> f32 {
    num as f32 / max as f32 * 100_f32
}

#[wasm_bindgen]
fn grade_point(grade: i32, max: i32) -> f32 {
    let grade_percent = percent(grade, max);

    match grade_percent.round() as i32 {
      97 ... 100 => 4.00,
      94 ... 96  => 4.00,
      90 ... 93  => 3.70,
      87 ... 89  => 3.30,
      84 ... 86  => 3.00,
      80 ... 83  => 2.70,
      77 ... 79  => 2.30,
      74 ... 76  => 2.00,
      70 ... 73  => 1.70,
      67 ... 69  => 1.30,
      64 ... 66  => 1.00,
      60 ... 63  => 0.70,
      0  ... 59  => 0.00,
              _  => 0.00
    }
}

#[wasm_bindgen]
fn grade_point_ave(grades_json: &String, max: i32) -> f32 {
    let grades: Vec<i32> = serde_json::from_str(&grades_json).unwrap();
    
    let mut result = 0.00;
    
    for grade in &grades {
        result += grade_point(*grade, max);
    }
   
    result / grades.len() as f32
}

#[wasm_bindgen]
fn grade_letter(grade: i32, max: i32) -> String {
    let grade_percent = percent(grade, max);

    match grade_percent.round() as i32 {
      97 ... 100 => "A+".to_string(),
      94 ... 96  => "A".to_string(),
      90 ... 93  => "A-".to_string(),
      87 ... 89  => "B+".to_string(),
      84 ... 86  => "B".to_string(),
      80 ... 83  => "B-".to_string(),
      77 ... 79  => "C+".to_string(),
      74 ... 76  => "C".to_string(),
      70 ... 73  => "C-".to_string(),
      67 ... 69  => "D+".to_string(),
      64 ... 66  => "D".to_string(),
      60 ... 63  => "D-".to_string(),
      0  ... 59  => "F".to_string(),
              _  => "Undefined".to_string()
    }
}

fn main() {

}
