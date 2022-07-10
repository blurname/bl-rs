use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add_zwsp(name: &str) -> i32 {
    // so many string concatenate ways, which should I choose
    //
    // path1
    alert(&format!("heddllo,{}", 12));
    // let mut output = String::from("");
    // for ch in name.chars() {
    //     output.push(ch);
    //     output.push('\u{200b}')
    // }

    // // path2
    // let mut output = String::from("");
    // for ch in name.chars() {
    //     output.push_str(&(String::from(ch) + &String::from('\u{200b}')));
    // }
    // alert("hello");
    12
}
