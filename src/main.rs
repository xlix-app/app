// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::Manager;
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings6;
use windows::core::{Abi, Interface};


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut builder = tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::External("https://www.xlix.app".parse().unwrap()),
            );

            let window = builder
                .additional_browser_args("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection,ElasticOverscroll")
                .maximizable(true)
                .inner_size(800.0, 500.0)
                .min_inner_size(800.0, 500.0)
                .resizable(true)
                .fullscreen(false)
                .title("xlix-app")
                .build()?;

            window.with_webview(|webview| unsafe {
                let settings = webview.controller().CoreWebView2().unwrap().Settings().unwrap();
                let settings = settings.cast::<ICoreWebView2Settings6>().unwrap();
                settings.SetIsSwipeNavigationEnabled(false).unwrap();
                settings.SetIsPinchZoomEnabled(false).unwrap();
                settings.SetIsZoomControlEnabled(false).unwrap();
            }).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running the application");
}
