
use wasm_bindgen::prelude::*;
use leptos::*;

#[component]
pub fn hello_world(cx: Scope) -> impl IntoView {

    view! {cx,
        <h1> "Hello World" </h1>
    }
}

#[wasm_bindgen]
pub fn gen() -> Result<JsValue, JsError> {

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
/*
    let window = web_sys::window().ok_or_else(|| {
        return JsError::new("could not find window");
    })?;

    let document = window.document().ok_or_else(|| {
        return JsError::new("could not find document");
    })?;

    let body = document.body().ok_or_else(||{
        return JsError::new("could not find body");
    })?;
*/
    let runtime = create_runtime();
    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx, <HelloWorld/>
        }
    });


    runtime.dispose();
    Ok(JsValue::from_str(&html))
}

