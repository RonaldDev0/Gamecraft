use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, WgpuSettings},
    },
    window::{
        PrimaryWindow,
        // WindowMode::BorderlessFullscreen,
        PresentMode::AutoVsync,
        CursorGrabMode,
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
        .add_systems(Startup, (set_window_icon, setup_window))
        .add_systems(Update, toggle_cursor);
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

fn setup_window(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }
}

fn toggle_cursor(
    mut windows: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.single_mut() {
            // Toggle cursor grab mode
            if window.cursor_options.visible {
                window.cursor_options.visible = false;
                window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Confined;
            } else {
                window.cursor_options.visible = true;
                window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
            }
        }
    }
}