use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn js_sucks(cx: Scope) -> impl IntoView {
    view! {cx,
        <>
        <h1> "JavaScript Sucks!" </h1>
        <p> "Built with Leptos" </p>
        </>
    }
}

#[wasm_bindgen]
pub fn gen() -> Result<JsValue, JsError> {
    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx, <><JsSucks/></>
        }
    });

    Ok(JsValue::from_str(&html))
}
