mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasmiodd!");
}

#[wasm_bindgen]
pub async fn getidentifier(port :i32) -> Result<JsValue, JsValue> {

    let c = iotcoreconnect::iotcoreclient::HttpClient::new("http://192.168.56.89".to_string());
    let (vendor, deviceid, productname) = c.get_device_identifier(2).await;
    Ok(JsValue::from_serde(&vendor).unwrap())
}
