use crate::components::{Position, Size};
use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct SnakeHead;

pub fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 5, y: 5 })
        .insert(Size::square(0.8));
}

pub fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<(&SnakeHead, &mut Transform)>,
) {
    for (_head, mut transform) in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 1.;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn entity_has_snake_head() {
        // Arrange
        let mut app = App::new();
        app.add_systems(Startup, spawn_snake);
        app.update();

        // Act
        let mut query = app.world.query_filtered::<Entity, With<SnakeHead>>();

        // Assert
        assert_eq!(query.iter(&app.world).count(), 1);
    }

    #[test]
    fn snake_head_has_moved_up() {
        // Arrange
        let mut app = App::new();
        let default_transform = Transform { ..default() };

        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, snake_movement);

        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW);
        app.insert_resource(input);

        // Act
        app.update();
        let mut query = app.world.query::<(&SnakeHead, &Transform)>();

        // Assert
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert!(default_transform.translation.y < transform.translation.y);
            assert_eq!(default_transform.translation.x, transform.translation.x);
        })
    }

    #[test]
    fn snake_head_has_moved_up_and_right() {
        // Arrange
        let mut app = App::new();
        let default_transform = Transform { ..default() };
        let mut up_transform = Transform { ..default() };

        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, snake_movement);

        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW);
        app.insert_resource(input);

        // Act
        app.update();
        let mut query = app.world.query::<(&SnakeHead, &Transform)>();

        // Assert
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert!(default_transform.translation.y < transform.translation.y);
            assert_eq!(default_transform.translation.x, transform.translation.x);
            up_transform = transform.to_owned();
        });

        // Arrange
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyD);
        app.insert_resource(input);

        // Act
        app.update();
        let mut query = app.world.query::<(&SnakeHead, &Transform)>();

        // Assert
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert_eq!(up_transform.translation.y, transform.translation.y);
            assert!(up_transform.translation.x < transform.translation.x);
        })
    }

    #[test]
    fn snake_head_moves_down_and_left() {
        // Setup
        let mut app = App::new();
        let default_transform = Transform { ..default() };

        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, snake_movement);

        // Movimenta para baixo
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyS);
        app.insert_resource(input);
        app.update();

        // Movimenta para esquerda
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyA);
        app.insert_resource(input);
        app.update();

        // Assert
        let mut query = app.world.query::<(&SnakeHead, &Transform)>();
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert!(default_transform.translation.y > transform.translation.y);
            assert!(default_transform.translation.x > transform.translation.x);
        })
    }
}
