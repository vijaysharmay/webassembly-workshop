extern crate typed_html;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use typed_html::{html, dom::DOMTree};

#[wasm_bindgen]
pub fn render() -> String {
    let index: DOMTree<String> = html!(
        <html>
            <head>
                <title>"Typed HTML Demo"</title>
            </head>
            <body>
                <h1>"Hello world from Typed HTML Template"</h1>
            </body>
        </html>
    );
    index.to_string()
}



