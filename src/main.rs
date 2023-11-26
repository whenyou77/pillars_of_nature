use std::time::Duration;

use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy::sprite::collide_aabb::collide;
use bevy_ecs_ldtk::prelude::*;
// use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

const TILE_SIZE: f32 = 32.;

#[derive(Default)]
pub enum NpcType{
    Alpaca,
    #[default]
    Test
}
#[derive(Default)]
pub enum TotemType{
    Fire,
    Earth,
    Water,
    #[default]
    Air
}
#[derive(Default)]
pub enum Direction{
    #[default]
    North,
    South,
    East,
    West
}

#[derive(Component, Default)]
struct Player;
#[derive(Component, Default)]
struct Checkpoint
{
    id: u8,
    lit: bool
}
#[derive(Component, Default)]
struct CheckpointBundle{
    checkpoint: Checkpoint,
    sprite_bundle: SpriteBundle
}
#[derive(Component, Default)]
struct PlayerSpawn{
    affect_x: bool,
    affect_y: bool
}
#[derive(Component, Default)]
struct Coin;
#[derive(Component, Default)]
struct Door{
    id: String
}
#[derive(Component, Default)]
struct Trigger{
    id: u32,
    visible: bool
}
#[derive(Component, Default)]
struct Npc
{
    npc_type: NpcType,
    dialogue: String
}
#[derive(Component, Default)]
struct Enemy
{
    move_distance: Vec2,
    tangible: bool,
    smart: bool
}
#[derive(Component, Default)]
struct Solid;
#[derive(Component, Default)]
struct Killer(Direction);
#[derive(Bundle,LdtkIntCell)]
struct SolidBundle{
    solid: Solid,
    transform: Transform
}
#[derive(Bundle)]
struct KillerBundle{
    killer: Killer,
    transform: Transform
}
#[derive(Component, Default)]
struct Actor{
    grounded: bool,
    jumped: bool,
    coyote_time: u8,
    jump_count: u8,
    jump_limit: u8
}
#[derive(Component, Default)]
struct Vel(Vec2);
#[derive(Component)]
struct PlayerCamera;

#[derive(Bundle)]
struct PlayerBundle
{
    player: Player,
    vel: Vel,
    actor: Actor,
    sprite_bundle: SpriteBundle,
    worldly: Worldly
}

#[derive(Bundle)]
struct EnemyBundle
{
    enemy: Enemy,
    vel: Vel,
    actor: Actor,
    //#[sprite_bundle("monochrome_tilemap_transparent_packed.png")]
    sprite_bundle: SpriteBundle
}

#[derive(Bundle)]
struct PlayerSpawnBundle
{
    player_spawn: PlayerSpawn,
    transform: Transform
}

#[derive(Bundle, LdtkEntity)]
struct CoinBundle
{
    coin: Coin,
    vel: Vel,
    #[sprite_sheet_bundle(
        "monochrome_tilemap_transparent_packed.png",
        16.,
        16.,
        3,
        1,
        0.,
        0.,
        0
    )]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    /*#[sprite_bundle("ghost.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle*/
}

#[derive(Bundle)]
struct DoorBundle
{
    door: Door,
    sprite: SpriteBundle
}

#[derive(Bundle)]
struct TriggerBundle
{
    trigger: Trigger,
    sprite: SpriteBundle
}

#[derive(Component, Default)]
struct Totem(TotemType);

#[derive(Bundle)]
struct TotemBundle
{
    sprite_bundle: SpriteBundle,
    totem: Totem
}

#[derive(Component, Default)]
struct InGameText
{
    value: String,
    visible: bool,
    text_id: u32
}

#[derive(Bundle)]
struct InGameTextBundle
{
    text_bundle: Text2dBundle,
    text: InGameText
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState
{
    AssetLoading,
    Setup,
    MapLoad,
    Gameplay,
    Pause,
    Menu
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegState
{
    Idle,
    Running,
    Airborne
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyState
{
    Idle,
    Running,
    Airborne,
    Shooting
}

impl LdtkEntity for TriggerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> TriggerBundle {
        let sprite = Sprite {
            custom_size: Some(Vec2::new(entity_instance.width as f32,entity_instance.height as f32)),
            color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. },
            ..Default::default()
        };

        let mut id: u32 = 666;
        let mut visible = false;

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"ID")
        {
            if let FieldValue::Int(id_field) = field_instance.value {
                id = id_field.unwrap() as u32;
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Visible")
        {
            if let FieldValue::Bool(visible_field) = field_instance.value {
                visible = visible_field;
            }
        }

        TriggerBundle { trigger: Trigger{
            id: id,
            visible: visible
        }, 
        sprite: SpriteBundle{sprite: sprite,..default()} }
    }
}

impl LdtkEntity for DoorBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> DoorBundle {
        let sprite = Sprite {
            custom_size: Some(Vec2::new(entity_instance.width as f32,entity_instance.height as f32)),
            color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. },
            ..Default::default()
        };

        let mut lvid = String::new();

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"LvID")
        {
            if let FieldValue::String(lv_id) = field_instance.value.to_owned() {
                lvid = lv_id.unwrap();
            }
        }

        DoorBundle {
            door: Door{id: lvid},
            sprite: SpriteBundle{sprite: sprite,..default()}
        }
    }
}

impl LdtkEntity for InGameTextBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> InGameTextBundle {

        let mut value = String::new();
        let mut visible = false;
        let mut id: u32 = 666;

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Value")
        {
            if let FieldValue::String(value_field) = field_instance.value.to_owned() {
                value = value_field.unwrap();
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Visible")
        {
            if let FieldValue::Bool(visible_field) = field_instance.value {
                visible = visible_field;
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"ID")
        {
            if let FieldValue::Int(id_field) = field_instance.value {
                id = id_field.unwrap() as u32;
            }
        }

        InGameTextBundle {
            text_bundle: Text2dBundle{
                text: Text::from_section(value.clone(), TextStyle {
                    font: asset_server.load("Lato-Black.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                }).with_alignment(TextAlignment::CENTER),
                visibility: Visibility { is_visible: visible },
                ..default()
            },
            text: InGameText{
                value: value,
                visible: visible,
                text_id: id
            }
        }
    }
}

impl LdtkEntity for CheckpointBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> CheckpointBundle {
        
        let mut ch_id: u8 = 0;

        if let Some(door_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"ID")
        {
            if let FieldValue::Int(Some(ch_val)) = door_field.value {
                ch_id = ch_val as u8;
            }
        }

        CheckpointBundle
        {
            checkpoint: Checkpoint { id: ch_id, lit: false },
            sprite_bundle: SpriteBundle { sprite: Sprite{custom_size: Some(Vec2::new(16.,32.)),..default()}, ..default() }
        }
    }
}

impl LdtkEntity for PlayerSpawnBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> PlayerSpawnBundle {

        let mut affect = (false,false);

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"AffectX")
        {
            if let FieldValue::Bool(affector) = field_instance.value {
                affect.0 = affector;
            }
        }

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"AffectY")
        {
            if let FieldValue::Bool(affector) = field_instance.value {
                affect.1 = affector;
            }
        }

        PlayerSpawnBundle
        {
            player_spawn: PlayerSpawn {affect_x: affect.0, affect_y: affect.1},
            transform: Transform {..default()}
        }
    }
}

impl LdtkEntity for TotemBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> TotemBundle {
        
        let mut totem_string = String::new();
        let mut totem_type = TotemType::default();
        let mut totem_sprite: Handle<Image> = asset_server.load("air_totem.png");

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Totem")
        {
            if let FieldValue::Enum(t_type) = field_instance.value.to_owned() {
                totem_string = t_type.unwrap().to_lowercase();
            }
        }


        match totem_string.as_str()
        {
            "firetotem" => {
                totem_sprite = asset_server.load("fire_totem_32.png");
                totem_type = TotemType::Fire;
            },
            "earthtotem" => {
                totem_sprite = asset_server.load("earth_totem.png");
                totem_type = TotemType::Earth;
            },
            "watertotem" => {
                totem_sprite = asset_server.load("water_totem.png");
                totem_type = TotemType::Water;
            },
            _ => ()
        }

        TotemBundle
        {
            sprite_bundle: SpriteBundle { texture: totem_sprite , ..default()},
            totem: Totem(totem_type)
        }
    }
}

impl LdtkEntity for EnemyBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> EnemyBundle {

        let mut speed = Vec2::new(0.,0.);
        let mut distance = Vec2::new(0.,0.);
        let mut tangible = true;
        let mut smart = true;

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"MoveDistanceX")
        {
            if let FieldValue::Float(mdx_field) = field_instance.value {
                if mdx_field.is_some()
                {
                    distance.x = mdx_field.unwrap();
                }
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"MoveDistanceY")
        {
            if let FieldValue::Float(mdy_field) = field_instance.value {
                if mdy_field.is_some()
                {
                    distance.y = mdy_field.unwrap();
                }
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"MoveSpeedX")
        {
            if let FieldValue::Float(speedx_field) = field_instance.value {
                if speedx_field.is_some()
                {
                    speed.x = speedx_field.unwrap();
                }
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"MoveSpeedY")
        {
            if let FieldValue::Float(speedy_field) = field_instance.value {
                if speedy_field.is_some()
                {
                    speed.y = speedy_field.unwrap();
                }
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Tangible")
        {
            if let FieldValue::Bool(tangible_field) = field_instance.value {
                tangible = tangible_field;
            }
        }
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Smart")
        {
            if let FieldValue::Bool(smart_field) = field_instance.value {
                smart = smart_field;
            }
        }

        EnemyBundle
        {
            sprite_bundle: SpriteBundle { texture: asset_server.load("slime.png"), sprite: Sprite { color: Color::rgb_u8(255, 0, 0), custom_size: Some(Vec2::new(32.,32.)),..default()},..default()},
            enemy: Enemy { move_distance: distance, tangible: tangible, smart: smart },
            vel: Vel(Vec2::new(0.,0.)),
            actor: Actor { grounded: false, jumped: false, coyote_time: 0, jump_count: 0, jump_limit: 0 }
        }
    }
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> PlayerBundle {

        /*if let Some(door_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"LvID")
        {
            if let FieldValue::String(Some(lv_id)) = &door_field.value {
                lvid = lv_id.clone();
            }
        }*/

        PlayerBundle
        {
            player: Player,
            vel: Vel(Vec2::new(0.,0.)),
            actor: Actor
            {
                grounded: false,
                jumped: false,
                coyote_time: 0,
                jump_count: 0,
                jump_limit: 1
            },
            sprite_bundle: SpriteBundle{texture: asset_server.load("protagonist_silhouette.png"), sprite: Sprite { custom_size: Some(Vec2::new(16.,32.)), ..default()}, ..default()},
            worldly: Worldly { entity_iid: entity_instance.iid.to_owned() }
        }
    }
}

impl LdtkIntCell for KillerBundle
{
    fn bundle_int_cell(int_grid_cell: IntGridCell, _: &LayerInstance) -> Self {
        match int_grid_cell.value
        {
            2 => KillerBundle { killer: Killer(Direction::North), transform: Transform::default() },
            3 => KillerBundle { killer: Killer(Direction::South), transform: Transform::default() },
            4 => KillerBundle { killer: Killer(Direction::East), transform: Transform::default() },
            5 => KillerBundle { killer: Killer(Direction::West), transform: Transform::default() },
            _ => KillerBundle { killer: Killer(Direction::North), transform: Transform::default() }
        }
    }
}

/*#[derive(AssetCollection, Resource)]
struct TilemapAssets {
    #[asset(path = "test_32.ldtk")]
    world*_0: Handle<LdtkAsset>
}*/

fn main() {
    
    let mut app = App::new();
    app.add_loopless_state(GameState::Setup)
    /* .add_loading_state(LoadingState::new(GameState::AssetLoading)
        .continue_to_state(GameState::Setup)
        .with_collection::<TilemapAssets>())*/

    .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
            title: "PILLARS OF NATURE".to_string(),
            width: 512.,
            height: 512.,
            resizable: false,
            ..default()
        },
        ..default()
        }).set(ImagePlugin::default_nearest()))//.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin))

        .add_enter_system(GameState::Setup,setup)

        //.add_startup_system(go)

         .insert_resource(LdtkSettings{
            level_background: LevelBackground::Nonexistent,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb_u8(16, 0, 16)))

        .add_plugin(FramepacePlugin).insert_resource(FramepaceSettings{limiter: Limiter::Manual(Duration::from_secs_f32(1./60.))})
        .add_plugin(LdtkPlugin)
        //.add_loopless_state(GameState::Gameplay)
        
        .insert_resource(LevelSelection::Identifier("Yard".to_string()))
        .register_ldtk_int_cell_for_layer::<SolidBundle>("IntGrid",1)
        .register_ldtk_int_cell_for_layer::<KillerBundle>("IntGrid",2)
        .register_ldtk_int_cell_for_layer::<KillerBundle>("IntGrid",3)
        .register_ldtk_int_cell_for_layer::<KillerBundle>("IntGrid",4)
        .register_ldtk_int_cell_for_layer::<KillerBundle>("IntGrid",5)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<CoinBundle>("Dubloon")
        .register_ldtk_entity::<DoorBundle>("Door")
        .register_ldtk_entity::<TotemBundle>("Totem")
        .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_entity::<InGameTextBundle>("TextEntity")

         /* .add_system_set(ConditionSet::new().run_in_bevy_state(GameState::Gameplay)
            .with_system(player_move)
            .with_system(actor_physics)
            .with_system(camera)
            .into()
        )*/

        .add_system(map_spawn.run_in_state(GameState::MapLoad))
        .add_system(player_move.run_in_state(GameState::Gameplay))
        .add_system(actor_physics.run_in_state(GameState::Gameplay))
        .add_system(coin.run_in_state(GameState::Gameplay))
        .add_system(door.run_in_state(GameState::Gameplay))
        .add_system(trigger.run_in_state(GameState::Gameplay))
        .add_system(killer.run_in_state(GameState::Gameplay))
        .add_system(totem.run_in_state(GameState::Gameplay))
        .add_system(text.run_in_state(GameState::Gameplay))
        .add_system(enemy_react.run_in_state(GameState::Gameplay))
        .add_system(camera.run_in_state(GameState::Gameplay))
        
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default()).insert(PlayerCamera);
    
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test_32.ldtk"),
        transform: Transform{
            scale: Vec3::new(1.,1.,1.),
            translation: Vec3::new(0.,-256.,0.),
            ..default()
        },
        
        ..Default::default()
    });

    commands.insert_resource(NextState(GameState::MapLoad));
}

fn map_spawn(mut commands: Commands, ldtk_event: EventReader<LevelEvent>,mut player_q: Query<&mut Transform, With<Player>>,player_spawn_q: Query<(&Transform, &PlayerSpawn), (With<PlayerSpawn>, Without<Player>)>, level_query: Query<
(&Transform, &Handle<LdtkLevel>),
(Without<PlayerSpawn>,Without<Player>),
>,
ldtk_levels: Res<Assets<LdtkLevel>>, mut visible_q: Query<(&mut Visibility, &InGameText), With<InGameText>>)
{
    if !ldtk_event.is_empty()
    {
        if !player_spawn_q.is_empty()
        {
            for mut p_transform in player_q.iter_mut()
            {
                let (ps_transform, ps) = player_spawn_q.single();
                for (level_transform, level_handle) in &level_query {
                    if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                        if ps.affect_x
                        {
                            p_transform.translation.x = ps_transform.translation.x+level_transform.local_x().x;
                        }
                        if ps.affect_y
                        {
                            p_transform.translation.y = ps_transform.translation.y+level_transform.local_y().y;
                        }
                    }
                }
            }
        }
        
        for (mut visible, text) in visible_q.iter_mut()
        {
            visible.is_visible = false;
            if text.visible
            {
                visible.is_visible = true;
            }
        }

        commands.insert_resource(NextState(GameState::Gameplay));
    }
}

fn player_move(mut player: Query<(&mut Vel, &mut Actor), With<Player>>, kb: Res<Input<KeyCode>>)
{
    let mut pid: u8 = 0;
    for (mut p_vel, mut p_actor) in player.iter_mut()
    {
        pid += 1;
        if pid == 1
        {
            if kb.pressed(KeyCode::D)
            {
                p_vel.0.x = 4.;
            }
            else if kb.pressed(KeyCode::A)
            {
                p_vel.0.x = -4.;
            }
            else
            {
                p_vel.0.x = 0.;
            }
            if kb.just_pressed(KeyCode::Space) && (p_actor.coyote_time > 0 || p_actor.jump_count > 0)
            {
                p_vel.0.y = 14.;
                p_actor.jumped = true;
                p_actor.jump_count -= 1;
                p_actor.coyote_time = 0;
            }
            if kb.just_released(KeyCode::Space) && p_vel.0.y > 3. && p_actor.jumped
            {
                p_vel.0.y = 3.;
            }
        }
        if pid == 2
        {
            if kb.pressed(KeyCode::Numpad6)
            {
                p_vel.0.x = 4.;
            }
            else if kb.pressed(KeyCode::Numpad4)
            {
                p_vel.0.x = -4.;
            }
            else
            {
                p_vel.0.x = 0.;
            }
            if kb.just_pressed(KeyCode::Numpad0) && p_actor.coyote_time > 0 && p_actor.jump_count > 0
            {
                p_vel.0.y = 14.;
                p_actor.jumped = true;
                p_actor.coyote_time = 0;
                p_actor.jump_count -= 1;
            }
            if kb.just_released(KeyCode::Numpad0) && p_vel.0.y > 3. && p_actor.jumped
            {
                p_vel.0.y = 3.;
            }
        }
    }
}

fn actor_physics(
    mut actor_q: Query<(&mut Vel, &mut Transform, &mut Actor, &Handle<Image>, Entity, &Sprite), With<Actor>>, 
    solid_q: Query<&Transform, (With<Solid>,Without<Actor>)>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<Actor>,Without<Solid>),
    >,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    images: Res<Assets<Image>>,
    mut commands: Commands)
{
    for (mut a_vel, mut a_transform, mut actor, a_image, a_entity, a_sprite) in actor_q.iter_mut()
    {
        if actor.grounded
        {
            actor.coyote_time = 5;
            actor.jump_count = actor.jump_limit;
        }
        else
        {
        if actor.coyote_time > 0
        {
            actor.jump_count = actor.jump_limit;
        }
        else if actor.jump_count == actor.jump_limit && actor.jump_count != 0
        {
            actor.jump_count = actor.jump_limit-1;
        }
        }
        
        actor.grounded = false;
        if actor.coyote_time > 0
        {
            actor.coyote_time -= 1;
        }

        let mut a_size = a_sprite.custom_size.unwrap();
        if a_sprite.custom_size.is_none()
        {
            a_size = images.get(a_image).unwrap().size();
        }
        

        if a_vel.0.y > -12.
        {
            a_vel.0.y -= 1.;
        }
        
        for s_transform in solid_q.iter()
        {
            if collide(
                a_transform.translation+Vec3::new(a_vel.0.x-TILE_SIZE/2.,-TILE_SIZE/2.,0.),
                Vec2::new(a_size.x,a_size.y),
                s_transform.translation,
                Vec2::splat(TILE_SIZE)
            ).is_some()
            {
                if a_transform.translation.x-TILE_SIZE/2. > s_transform.translation.x
                {
                    a_transform.translation.x -= (a_transform.translation.x-a_size.x/2.-TILE_SIZE/2.)-(s_transform.translation.x+TILE_SIZE/2.);//-TILE_SHIFT;
                    a_vel.0.x = 0.;
                }
                else if a_transform.translation.x-TILE_SIZE/2. < s_transform.translation.x
                {
                    a_transform.translation.x -= (a_transform.translation.x+a_size.x/2.-TILE_SIZE/2.)-(s_transform.translation.x-TILE_SIZE/2.);//-TILE_SHIFT;
                    a_vel.0.x = 0.;
                }
            }
        }
        for s_transform in solid_q.iter()
        {
            if collide(
                a_transform.translation+Vec3::new(-TILE_SIZE/2.,a_vel.0.y-TILE_SIZE/2.,0.),
                Vec2::new(a_size.x,a_size.y),
                s_transform.translation,
                Vec2::splat(TILE_SIZE)
            ).is_some()
            {
                if a_transform.translation.y-TILE_SIZE/2. > s_transform.translation.y
                {
                    a_transform.translation.y -= (a_transform.translation.y-a_size.y/2.-TILE_SIZE/2.)-(s_transform.translation.y+TILE_SIZE/2.)/*-TILE_SHIFT*/;
                    a_vel.0.y = 0.;
                    actor.grounded = true;
                    actor.jumped = false;
                }
                else if a_transform.translation.y-TILE_SIZE/2. < s_transform.translation.y
                {
                    a_transform.translation.y -= (a_transform.translation.y+a_size.y/2.-TILE_SIZE/2.)-(s_transform.translation.y-TILE_SIZE/2.)/*-TILE_SHIFT*/;
                    a_vel.0.y = 0.;
                }
            }
        }
        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                    if a_transform.translation.x+a_vel.0.x+a_size.x/2. > level.px_wid as f32+level_transform.local_x().x
                    {
                        a_transform.translation.x = level.px_wid as f32+level_transform.local_x().x-a_vel.0.x-a_size.x/2.;
                    }
                    if a_transform.translation.x+a_vel.0.x-a_size.x/2. < level_transform.local_x().x
                    {
                        a_transform.translation.x = level_transform.local_x().x-a_vel.0.x+a_size.x/2.;
                    }
                    if a_transform.translation.y+a_vel.0.y+a_size.y/2. < level_transform.local_y().y
                    {
                        commands.entity(a_entity).despawn();
                    }
            }
        }

        a_transform.translation += a_vel.0.extend(0.);
    }
}

fn camera(mut camera_q: Query<&mut Transform, With<PlayerCamera>>, 
    player_q: Query<&Transform, (With<Player>,Without<PlayerCamera>)>, 
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<PlayerCamera>, Without<Player>),
    >,
    ldtk_levels: Res<Assets<LdtkLevel>>)
{
    for p_transform in player_q.iter()
    {
        camera_q.single_mut().translation.x = p_transform.translation.x;
        camera_q.single_mut().translation.y = p_transform.translation.y-256.;
        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if p_transform.translation.x > level.px_wid as f32-256.+level_transform.local_x().x
                {
                    camera_q.single_mut().translation.x = level.px_wid as f32-256.+level_transform.local_x().x;
                }
                else if p_transform.translation.x < 256.+level_transform.local_x().x
                {
                    camera_q.single_mut().translation.x = 256.+level_transform.local_x().x;
                }
                if p_transform.translation.y-256. > level.px_hei as f32/2.+level_transform.local_y().y-256.
                {
                    camera_q.single_mut().translation.y = level.px_hei as f32/2.+level_transform.local_y().y-256.;
                }
                else if p_transform.translation.y-256. < level_transform.local_y().y
                {
                    camera_q.single_mut().translation.y = level_transform.local_y().y;
                }
            }
        }
        break;
    }
    
}

fn door(player_q: Query<&Transform, With<Player>>,door: Query<(&Transform,&Door,&Sprite),(With<Door>,Without<Player>)>, mut commands: Commands)
{
    for p_transform in player_q.iter()
    {
        for (d_transform,door_id, d_sprite) in door.iter()
        {
            if collide(
                p_transform.translation,
               Vec2::new(16.,32.),
                d_transform.translation,
              d_sprite.custom_size.unwrap()
            ).is_some()
            {
                commands.insert_resource(LevelSelection::Identifier(door_id.id.to_owned()));
                commands.insert_resource(NextState(GameState::MapLoad));
            }
        }
    }
}

fn trigger(player_q: Query<&Transform, With<Player>>,trigger_q: Query<(&Transform,&Trigger,&Sprite),(With<Trigger>,Without<Player>)>, mut text_q: Query<(&Visibility, &mut InGameText), With<InGameText>>)
{
    for p_transform in player_q.iter()
    {
        for (t_transform,trigger_id, t_sprite) in trigger_q.iter()
        {
            if collide(
                p_transform.translation,
               Vec2::new(16.,32.),
                t_transform.translation,
              t_sprite.custom_size.unwrap()
            ).is_some()
            {
                for (tx_visible, mut tx_text) in text_q.iter_mut()
                {
                    if tx_text.text_id == trigger_id.id
                    {
                        tx_text.visible = trigger_id.visible;
                    }
                }
            }
        }
    }
}

fn text(mut text_q: Query<(&mut Visibility, &InGameText), With<InGameText>>)
{
    for (mut tx_visible, tx_text) in text_q.iter_mut()
    {
        tx_visible.is_visible = tx_text.visible;
    }
}

fn killer(player_q: Query<(&Transform, &Vel, Entity), With<Player>>,killer_q: Query<(&Transform, &Killer),(With<Killer>,Without<Player>)>, mut commands: Commands)
{
    for (p_transform, p_vel, p_entity) in player_q.iter()
    {
        for (k_transform, k_stats) in killer_q.iter()
        {
            let mut k_size = Vec2::new(32.,14.);
            let mut k_delta = Vec2::new(16.,7.);
            match k_stats.0
            {
                Direction::East => {
                    k_size = Vec2::new(14.,32.);
                    k_delta = Vec2::new(7.,16.);
                },
                Direction::West => {
                    k_size = Vec2::new(14.,32.);
                    k_delta = Vec2::new(21.,16.);
                },
                Direction::North => (),
                Direction::South => {
                    k_delta = Vec2::new(16.,21.);
                },
            }
            if collide(
                p_transform.translation+p_vel.0.extend(0.),
                Vec2::new(16.,32.),
                k_transform.translation+k_delta.extend(0.),
                k_size
            ).is_some()
            {
                commands.entity(p_entity).despawn();
            }
        }
    }
}

fn coin(player_q: Query<&Transform, With<Player>>,coin_q: Query<(&Transform,Entity),(With<Coin>,Without<Player>)>, mut commands: Commands)
{
    for p_transform in player_q.iter()
    {
        for (c_transform, c_entity) in coin_q.iter()
        {
            if collide(
                p_transform.translation,
               Vec2::new(16.,32.),
                c_transform.translation,
                Vec2::splat(16.)
            ).is_some()
            {
                commands.entity(c_entity).despawn();
            }
        }
    }
}

//fn after_death(player_q: Query<&mut Transform>)

fn totem(mut player_q: Query<(&Transform, &mut Actor, &Vel), With<Player>>,totem_q: Query<(&Transform, &Totem, Entity), (Without<Player>, With<Totem>)>, mut commands: Commands)
{
    for (p_transform, mut p_actor, p_vel) in player_q.iter_mut()
    {
        for (t_transform, t_type, t_entity) in totem_q.iter()
        {
            if collide(
                p_transform.translation+p_vel.0.extend(0.),
               Vec2::new(16.,32.),
                t_transform.translation,
                Vec2::splat(32.)
            ).is_some()
            {
                match t_type.0
                {
                    TotemType::Air => p_actor.jump_limit += 1,
                    _ => ()
                }
                commands.entity(t_entity).despawn();
            }
        }
    }
}

fn enemy_react(mut player_q: Query<(&Transform, &mut Actor, &mut Vel, Entity), With<Player>>,enemy_q: Query<(&Transform, &Enemy, Entity), (Without<Player>, With<Enemy>)>, mut commands: Commands)
{
    for (p_transform, mut p_actor, mut p_vel, p_entity) in player_q.iter_mut()
    {
        for (e_transform, e_stats, e_entity) in enemy_q.iter()
        {
            if collide(
                p_transform.translation+Vec3::new(0.,p_vel.0.y,0.),
               Vec2::new(16.,32.),
                e_transform.translation,
                Vec2::splat(32.)
            ).is_some()
            {
                if p_vel.0.y < 0. && p_transform.translation.y-16. >= e_transform.translation.y+16. && e_stats.tangible
                {
                    commands.entity(e_entity).despawn();
                    p_vel.0.y = 14.;
                    p_actor.jumped = false;
                    p_actor.jump_count += p_actor.jump_limit-1;
                }
                else {
                    commands.entity(p_entity).despawn();
                    println!("PLAYERX: {}\nPLAYERY: {}\nENEMYX: {}\nENEMYY: {}", p_transform.translation.x,p_transform.translation.y,e_transform.translation.x,e_transform.translation.y)
                }
            }
            if collide(
                p_transform.translation+Vec3::new(p_vel.0.x,0.,0.),
               Vec2::new(16.,32.),
                e_transform.translation,
                Vec2::splat(32.)
            ).is_some()
            {
                commands.entity(p_entity).despawn();
                println!("PLAYERX: {}\nPLAYERY: {}\nENEMYX: {}\nENEMYY: {}", p_transform.translation.x,p_transform.translation.y,e_transform.translation.x,e_transform.translation.y)
            }
        }
    }
}