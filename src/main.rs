use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_snake, setup))
        .add_systems(
            Update,
            (print_names, take_damage, despawn_dead, snake_movement).chain(),
        )
        .run();
}

#[derive(Debug, Component)]
struct Health(f32);

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Component)]
struct SnakeHead {
    direction: Direction,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn print_names(names: Query<(&Name, &Health)>) {
    for (name, health) in names.iter() {
        println!("hi there, {name} ({health})", health = health.0);
    }
}

fn take_damage(mut enemies: Query<&mut Health, With<SnakeHead>>) {
    for mut health in enemies.iter_mut() {
        health.0 = (health.0 - 10.0).max(0.0);
    }
}

fn despawn_dead(mut commands: Commands, healths: Query<(Entity, &Health)>) {
    for (entity, health) in healths.iter() {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_snake(mut commands: Commands) {
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
        .insert(SnakeHead {
            direction: Direction::Up,
        });
}

fn snake_movement(
    keybord_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<(&mut SnakeHead, &mut Transform)>,
) {
    let _ = keybord_input;
    for (mut head, mut transform) in head_positions.iter_mut() {
        if keybord_input.pressed(KeyCode::ArrowLeft) {
            head.direction = Direction::Left;
        }
        if keybord_input.pressed(KeyCode::ArrowRight) {
            head.direction = Direction::Right;
        }
        if keybord_input.pressed(KeyCode::ArrowDown) {
            head.direction = Direction::Down;
        }
        if keybord_input.pressed(KeyCode::ArrowUp) {
            head.direction = Direction::Up;
        }
        match head.direction {
            Direction::Up => {
                transform.translation.y += 1.0;
            }
            Direction::Down => {
                transform.translation.y -= 1.0;
            }
            Direction::Left => {
                transform.translation.x -= 1.0;
            }
            Direction::Right => {
                transform.translation.x += 1.0;
            }
        }
    }
}
