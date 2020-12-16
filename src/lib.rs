use wasm_bindgen::prelude::*;
use js_sys::Map;
use js_sys::JsString;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn add(a: &Map) -> i32 {
    // use web_sys::console;
    let find_string = &"b0";
    let mut previous = Map::new();
    let mut counter = 0;
    a.for_each(&mut |value, key| {
        let v: Map = value.unchecked_into();
        let k: JsString = key.unchecked_into();
        if k.includes(find_string, 0) {
            counter += 1;
            // console::log_1(&"Found One".into());
            v.set(&"previous".into(), &previous);
        }
        previous = v;
    });

    return counter
}
