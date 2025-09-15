use eframe_template::TemplateApp;

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(
    app: egui_winit::winit::platform::android::activity::AndroidApp,
) -> Result<(), Box<dyn std::error::Error>> {
    use egui_winit::winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("eframe_template")
            .with_max_level(log::LevelFilter::Info),
    );
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));
    eframe::run_native(
        "eframe_template",
        options,
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    )?;

    Ok(())
}
