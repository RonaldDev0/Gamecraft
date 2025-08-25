use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, WgpuSettings},
    },
    window::{
        PrimaryWindow,
        // WindowMode::BorderlessFullscreen,
        PresentMode::AutoVsync
    },
    winit::WinitWindows,
};

use image;
use winit::window::Icon;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Gamecraft".to_string(),
                        resizable: false,
                        present_mode: AutoVsync,
                        // mode: BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                }),
        )
        .add_systems(Startup, set_window_icon);
    }
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    if let Ok(primary_entity) = primary_window.single() {
        if let Some(primary) = windows.get_window(primary_entity) {
            match image::load_from_memory(include_bytes!("../assets/icon.png")) {
                Ok(img) => {
                    let rgba = img.into_rgba8();
                    let (width, height) = rgba.dimensions();

                    match Icon::from_rgba(rgba.into_raw(), width, height) {
                        Ok(icon) => {
                            primary.set_window_icon(Some(icon));
                            info!("✅ Window icon set successfully");
                        }
                        Err(err) => {
                            warn!("⚠️ Failed to create window icon: {}", err);
                        }
                    }
                }
                Err(err) => {
                    warn!("⚠️ Could not load window icon: {}", err);
                }
            }
        }
    } else {
        warn!("⚠️ No primary window found, skipping icon setup");
    }
}
