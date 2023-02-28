use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_pancam::PanCam;

use crate::tween::TweenTransform;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            // .add_system(camera_follow)
            .add_system(player_movement)
            .add_system(player_input_debug)
            .add_event::<ProcessTurn>();
    }
}

#[derive(Component)]
pub struct Player;

pub struct ProcessTurn;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(32.)),
            material: materials.add(ColorMaterial::from(Color::CYAN)),
            ..default()
        })
        .insert(Player);
}

// fn camera_follow(
//     mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
//     player: Query<&Transform, (With<Player>, Without<Camera>)>,
// ) {
//     let mut camera = camera.single_mut();
//     let player = player.single();

//     camera.translation = player.translation;
// }

fn player_movement(
    mut turn_event: EventWriter<ProcessTurn>,
    mut player: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let mut player = player.single_mut();

    if input.just_pressed(KeyCode::H) {
        player.translation.x -= 32.;
        println!("POS: {:?}", player.translation);
    }
    if input.just_pressed(KeyCode::J) {
        player.translation.y -= 32.;
    }
    if input.just_pressed(KeyCode::K) {
        player.translation.y += 32.;
    }
    if input.just_pressed(KeyCode::L) {
        player.translation.x += 32.;
    }

    turn_event.send(ProcessTurn);
}

fn player_input_debug(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let (entity, transform) = query.single();

    if input.just_pressed(KeyCode::T) {
        commands.entity(entity).insert(TweenTransform::new(
            transform.to_owned(),
            Transform::from_translation(Vec3::new(64., 64., 0.)),
            1.,
        ));
    }
}
