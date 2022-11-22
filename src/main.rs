use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;

#[derive(Component)]
struct GameEntity;
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
            title: "Game off thing?".to_string(),
            width: 768.,
            height: 768.,
            resizable: false,
            ..default()
        },
        ..default()
        }))
        .add_plugin(FramepacePlugin)
        .add_startup_system(setup)
        .add_system(player_move)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
		sprite: Sprite{
			custom_size: Some(Vec2::new(32.,32.)),
			..default()
		},
        texture: asset_server.load("okay_wide.png"),
        ..default()
    })
    .insert(GameEntity)
    .insert(Player);
}

fn player_move(mut player: Query<&mut Transform, With<Player>>, kb: Res<Input<KeyCode>>)
{
    let mut p_transform = player.single_mut();

    if kb.pressed(KeyCode::D)
    {
        p_transform.translation.x += 2.;
    }
    else if kb.pressed(KeyCode::A)
    {
        p_transform.translation.x -= 2.;
    }
    if kb.pressed(KeyCode::W)
    {
        p_transform.translation.y += 2.;
    }
    else if kb.pressed(KeyCode::S)
    {
        p_transform.translation.y -= 2.;
    }
}