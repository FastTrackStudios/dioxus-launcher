#[cfg(feature = "desktop")]
use dioxus::prelude::*;
#[cfg(feature = "native")]
use dioxus_native::prelude::*;

use launcher_core::QueryEngine;
use launcher_ui::components::Launcher;
use launcher_ui::state::LauncherState;
use launcher_ui::theme::Theme;
use providers::{ApplicationsProvider, CalcProvider, DemoProvider, WorkflowProvider};

fn build_engine() -> QueryEngine {
    // Load workflow packs from bundled + user directories
    let mut loaded_packs = Vec::new();

    // Bundled packs (next to the executable or in the project)
    let bundled_dir = std::path::PathBuf::from(
        std::env::var("DIOXUS_LAUNCHER_PACKS")
            .unwrap_or_else(|_| "packs".into())
    );
    loaded_packs.extend(launcher_core::pack::scan_packs(&bundled_dir));

    // User packs
    let user_dir = launcher_core::pack::default_pack_dir();
    loaded_packs.extend(launcher_core::pack::scan_packs(&user_dir));

    tracing::info!(packs = loaded_packs.len(), "Loaded workflow packs");

    let engine = QueryEngine::builder()
        .max_results(50)
        .register_tags(|tags| {
            // Desktop — blue
            tags.register("desktop", "Desktop", "Desktop environment")
                .set_color("desktop", "#89b4fa")
                .register("desktop/applications", "Applications", "Desktop applications")
                .register("desktop/applications/development", "Development", "Dev tools")
                .register("desktop/applications/graphics", "Graphics", "Graphics apps")
                .register("desktop/applications/network", "Network", "Network apps")
                .register("desktop/applications/multimedia", "Multimedia", "Media apps")
                .register("desktop/applications/system", "System", "System tools")
                // Audio — green
                .register("audio", "Audio", "Audio production")
                .set_color("audio", "#a6e3a1")
                .register("audio/effects", "Effects", "Audio effects")
                .set_color("audio/effects", "#a6e3a1")
                .register("audio/effects/reverb", "Reverb", "Reverb effects")
                .register("audio/effects/delay", "Delay", "Delay effects")
                .register("audio/effects/eq", "EQ", "Equalizers")
                .register("audio/effects/dynamics", "Dynamics", "Compressors, gates, etc")
                .register("audio/effects/plugin", "Plugins", "Plugin formats")
                .register("audio/effects/plugin/vst", "VST", "VST2 plugins")
                .register("audio/effects/plugin/vst3", "VST3", "VST3 plugins")
                .register("audio/instruments", "Instruments", "Virtual instruments")
                .set_color("audio/instruments", "#94e2d5")
                .register("audio/instruments/synth", "Synth", "Synthesizers")
                .register("audio/instruments/sampler", "Sampler", "Samplers")
                .register("audio/instruments/piano", "Piano", "Piano instruments")
                // Reaper — peach
                .register("reaper", "Reaper", "Reaper DAW")
                .set_color("reaper", "#fab387")
                .register("reaper/actions", "Actions", "Reaper actions")
                .set_color("reaper/actions", "#fab387")
                .register("reaper/actions/file", "File", "File actions")
                .register("reaper/actions/transport", "Transport", "Transport controls")
                .register("reaper/tracks", "Tracks", "Reaper tracks")
                .set_color("reaper/tracks", "#f5c2e7")
                // Tools — yellow
                .register("tools", "Tools", "Utility tools")
                .set_color("tools", "#f9e2af")
                .register("tools/calculator", "Calculator", "Math calculator")
                // Aliases
                .alias("apps", "desktop/applications")
                .alias("fx", "audio/effects")
                .alias("vst", "audio/effects/plugin/vst")
                .alias("inst", "audio/instruments")
                .alias("act", "reaper/actions")
                .alias("calc", "tools/calculator");
        })
        // Magic words: keyword + Space loads a preset
        .magic_word("C", "Compressors")
        .magic_word("R", "Reverbs")
        .magic_word("I", "Instruments")
        .magic_word("A", "Actions")
        // Create workflow provider from packs (extracts items)
        .register_packs(&loaded_packs)
        .provider(Box::new(DemoProvider::new()))
        .provider(Box::new(ApplicationsProvider::new()))
        .provider(Box::new(CalcProvider::new()))
        .provider(Box::new(WorkflowProvider::from_items(
            loaded_packs.iter().flat_map(|p| p.to_items()).collect()
        )))
        .build();
    // Register presets from packs
    for pack in &loaded_packs {
        for preset in &pack.def.presets {
            if engine.load_preset(&preset.name).is_none() {
                engine.save_preset(&preset.name, launcher_core::FilterState {
                    include: preset.include.clone(),
                    exclude: preset.exclude.clone(),
                    ..Default::default()
                });
            }
        }
    }

    // Seed some default presets for magic words to reference
    use launcher_core::FilterState;
    if engine.load_preset("Compressors").is_none() {
        engine.save_preset("Compressors", FilterState {
            include: vec!["audio/effects/dynamics".into()],
            ..Default::default()
        });
    }
    if engine.load_preset("Reverbs").is_none() {
        engine.save_preset("Reverbs", FilterState {
            include: vec!["audio/effects/reverb".into()],
            ..Default::default()
        });
    }
    if engine.load_preset("Instruments").is_none() {
        engine.save_preset("Instruments", FilterState {
            include: vec!["audio/instruments".into()],
            ..Default::default()
        });
    }
    if engine.load_preset("Actions").is_none() {
        engine.save_preset("Actions", FilterState {
            include: vec!["reaper/actions".into()],
            ..Default::default()
        });
    }

    engine
}

fn app() -> Element {
    let state = use_signal(|| LauncherState::new(build_engine()));

    let on_close = |_: ()| close_window();

    rsx! {
        Launcher { state, theme: Theme::dark(), on_close: on_close }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let engine = build_engine();
    tracing::info!(providers = engine.provider_names().len(), "Engine initialized");
    drop(engine);

    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::new().with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title("Launcher")
                        .with_inner_size(dioxus::desktop::LogicalSize::new(800.0, 520.0))
                        .with_decorations(false)
                        .with_always_on_top(true)
                        .with_resizable(true),
                ),
            )
            .launch(app);
    }

    #[cfg(feature = "native")]
    {
        use std::any::Any;

        let window_attrs = winit::window::WindowAttributes::default()
            .with_title("Launcher")
            .with_surface_size(winit::dpi::LogicalSize::new(800.0, 520.0))
            .with_decorations(false)
            .with_resizable(true);

        let configs: Vec<Box<dyn Any>> = vec![Box::new(window_attrs)];
        dioxus_native::launch_cfg(app, Vec::new(), configs);
    }
}

fn close_window() {
    std::process::exit(0);
}
