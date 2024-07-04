use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_snake, setup))
        .add_systems(Update, (print_names, take_damage, despawn_dead).chain())
        .run();
}

#[derive(Debug, Component)]
struct Health(f32);

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

#[derive(Debug, Component)]
struct SnakeHead;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((Name::new("J.C. Denton"), Health(100.0)));
    commands.spawn((Name::new("Bob Page"), Health(200.0), SnakeHead));
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
        .insert(SnakeHead);
}
