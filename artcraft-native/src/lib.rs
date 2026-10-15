pub mod app;
mod backend;
mod feed;
mod models;
mod overlays;
mod pages;
mod prompt_box;
mod shell;
mod theme;
mod ui;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: winit::platform::android::activity::AndroidApp) {
  android_logger::init_once(
    android_logger::Config::default().with_max_level(log::LevelFilter::Info),
  );
  let options = eframe::NativeOptions {
    android_app: Some(android_app),
    ..Default::default()
  };
  let _ = eframe::run_native(
    "ArtCraft",
    options,
    Box::new(|cc| Ok(Box::new(app::ArtcraftApp::new(cc)))),
  );
}
