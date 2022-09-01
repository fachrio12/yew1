mod app;
mod router;
mod pages;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
   
    yew::start_app::<app::Main>();


    Ok(())


   

}


