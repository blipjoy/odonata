use crate::bitmap::{Bitmap, BitmapPlugin};
use bevy::prelude::*;
use bevy_pixels::*;
use pix::{
    chan::{Ch8, Channel as _},
    el::Pixel,
    rgb::Rgba8p,
    Raster,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct CameraPlugin;

#[derive(Debug)]
pub struct FadePlugin;

/// The `Camera` resource offers methods for getting and setting the viewport transformation matrix
/// and size, and for accessing the internal pixel rasterizer.
pub struct Camera {
    viewport: Viewport,
    raster: Raster<Rgba8p>,
}

#[derive(Debug)]
struct Viewport {
    transform: Transform,
    size: Vec2,
}

/// Adding this component will cause the entity's [`Transform`] to be interpreted in screen space.
#[derive(Component)]
pub struct ScreenSpace;

#[derive(Component, Debug)]
pub struct Fade {
    timer: Timer,
    from: f32,
    to: f32,
    base_color: Rgba8p,
}

#[derive(Bundle)]
pub struct FadeBundle {
    fade: Fade,
    bitmap: Bitmap,
    transform: Transform,
    screen_space: ScreenSpace,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let PixelsOptions { width, height } = *app.world.resource::<PixelsOptions>();

        let viewport = Viewport {
            transform: Transform::identity(),
            size: Vec2::new(width as f32, height as f32),
        };
        let raster = Raster::<Rgba8p>::with_clear(width, height);

        app.insert_resource(Camera { viewport, raster })
            .add_plugin(PixelsPlugin)
            .add_plugin(BitmapPlugin)
            .add_plugin(FadePlugin);
    }
}

impl Camera {
    /// Get the viewport transformation matrix.
    pub fn transform(&self) -> Transform {
        self.viewport.transform
    }

    /// Get a mutable reference to the viewport transformation matrix.
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.viewport.transform
    }

    /// Get the viewport size.
    pub fn size(&self) -> &Vec2 {
        &self.viewport.size
    }

    /// Get a mutable reference to the camera's internal rasterizer.
    pub fn raster_mut(&mut self) -> &mut Raster<Rgba8p> {
        &mut self.raster
    }

    /// Create a component bundle that fades the entire viewport in.
    ///
    /// I.e. the entire viewport is cleared to the given base color which fades to transparent over
    /// time.
    pub fn fade_in(time_seconds: f32, width: u32, height: u32, base_color: Rgba8p) -> FadeBundle {
        let bitmap = Bitmap::clear_color(width, height, base_color);

        let timer = Timer::from_seconds(time_seconds, false);
        let from = 1.0;
        let to = 0.0;
        let fade = Fade {
            timer,
            from,
            to,
            base_color,
        };

        let transform = Transform::from_xyz(0.0, 0.0, std::f32::INFINITY);
        let screen_space = ScreenSpace;

        FadeBundle {
            bitmap,
            fade,
            transform,
            screen_space,
        }
    }

    /// Create a component bundle that fades the entire viewport out.
    ///
    /// I.e. the entire viewport is fades to the given base color over time.
    pub fn fade_out(time_seconds: f32, width: u32, height: u32, base_color: Rgba8p) -> FadeBundle {
        let bitmap = Bitmap::clear(width, height);

        let timer = Timer::from_seconds(time_seconds, false);
        let from = 0.0;
        let to = 1.0;
        let fade = Fade {
            timer,
            from,
            to,
            base_color,
        };

        let transform = Transform::from_xyz(0.0, 0.0, std::f32::INFINITY);
        let screen_space = ScreenSpace;

        FadeBundle {
            bitmap,
            fade,
            transform,
            screen_space,
        }
    }
}

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::update);
    }
}

impl FadePlugin {
    fn update(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Bitmap, &mut Fade)>,
        time: Res<Time>,
    ) {
        for (entity, mut bitmap, mut fade) in query.iter_mut() {
            if fade.timer.finished() {
                commands.entity(entity).despawn_recursive();
                continue;
            }

            fade.timer.tick(time.delta());

            let mut color = fade.base_color;

            // Apply the fade to the color (pre-multiplied alpha).
            let alpha =
                Ch8::from(fade.from).lerp(Ch8::from(fade.to), Ch8::from(fade.timer.percent()));
            for chan in color.channels_mut() {
                *chan = *chan * alpha;
            }

            // Force-update the bitmap raster.
            let raster = bitmap.raster_mut();
            *raster = Arc::new(Raster::with_color(raster.width(), raster.height(), color));
        }
    }
}
