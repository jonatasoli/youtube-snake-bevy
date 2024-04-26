use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::Size;

const GRID_WIDTH: u8 = 10;
const GRID_HEIGHT: u8 = 10;

fn size_scaling(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = primary_window.get_single().unwrap();
    for (sprite_size, mut transform) in &mut q.iter_mut() {
        scale_size(transform.as_mut(), sprite_size, window);
    }
}

fn scale_size(transform: &mut Transform, sprite_size: &Size, window: &Window) {
    transform.scale = Vec3::new(
        sprite_size.width / GRID_WIDTH as f32 * window.width(),
        sprite_size.height / GRID_HEIGHT as f32 * window.height(),
        1.,
    );
}

fn convert(pos: f32, bound_window: f32, grid_side_lenght: f32) -> f32 {
    let tile_size = bound_window / grid_side_lenght;
    pos / grid_side_lenght * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::Size;
    use bevy::window::WindowResolution;

    #[test]
    fn transform_has_correct_scale_for_window() {
        // Arrange
        let expected_transform = Transform {
            scale: Vec3::new(20., 20., 1.),
            ..default()
        };
        let mut default_transform = Transform {
            scale: Vec3::new(2., 3., 4.),
            ..default()
        };
        let sprite_size = Size::square(1.);

        let window = Window {
            resolution: WindowResolution::new(200., 200.),
            ..default()
        };
        // Act
        scale_size(&mut default_transform, &sprite_size, &window);

        // Assert
        assert_eq!(default_transform, expected_transform);
    }

    #[test]
    fn convert_position_x_for_grid_width() {
        let x = convert(4., 400., GRID_WIDTH as f32);

        assert_eq!(x, -20.)
    }

    #[test]
    fn convert_position_y_for_grid_height() {
        let y = convert(5., 400., GRID_HEIGHT as f32);

        assert_eq!(y, 20.)
    }
}
