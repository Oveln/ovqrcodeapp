#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const APP_NAME: &str = "ovqrcodeapp";

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(APP_NAME, options,Box::new(|_cc|Box::new(ovqrcodeapp::QrcodeApp::new())))
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let runner = eframe::WebRunner::new();
        
        runner.start(
            "ovqrcodeapp", // hardcode it
            web_options,
            Box::new(|_cc|Box::new(ovqrcodeapp::QrcodeApp::new()))
        )
        .await
        .expect("failed to start eframe");
    });
}
