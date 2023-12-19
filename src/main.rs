use bevy::{prelude::*, window::*, render::camera::ScalingMode};
use std::fs;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const PLAYER_SPEED: f32 = 100.0;
const PLAYER_X_SIZE: f32 = 24.0;
const PLAYER_Y_SIZE: f32 = 24.0;

#[derive(Component)]
pub struct Player{
    pub speed: f32,
    pub name: String,
    pub level: i32,
    pub health: i32,
    pub tear_cooldown: f32,
    pub direction: Vec2,
    pub position: Vec2,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                //turn off image filtering for pixel style
                .set(ImagePlugin::default_nearest())
                //set window title and size
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Undertale prototype 1".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .build(), 
        )
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, tear_spawn_system)
        .add_systems(Update, tear_movement_system)
        .run();
}

//spawn the red heart sprite
fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let save_info = fs::read_to_string("save.txt");

    let stats = decode_save(save_info.unwrap());

    let player = Player{
        speed: PLAYER_SPEED,
        name: stats[0].clone(),
        level: stats[1].parse::<i32>().unwrap(),
        health: stats[2].parse::<i32>().unwrap(),
        tear_cooldown: 0.0,
        direction: Vec2::new(0.0, 0.0),
        position: Vec2::new(0.0, 0.0),
    };
    
    println!("{} {} {}", player.name, player.level, player.health);

    let mut camera = Camera2dBundle::default();
    
    
    camera.projection.scaling_mode = ScalingMode::AutoMin{
        min_width: WINDOW_WIDTH,
        min_height: WINDOW_HEIGHT,
    };

    commands.spawn(camera);

    let texture_handle = asset_server.load("sprites/red_heart.png");

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(PLAYER_X_SIZE, PLAYER_Y_SIZE)),
            ..default()
        },
        texture: texture_handle,
        transform: Transform {
            translation: Vec3::new(player.position.x, player.position.y, 0.5),
            ..default()
        },
        ..default()
    }, player));
}

fn movement(
    //calls only entities with both Transform and PLayer components
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    //loops over all entities that match query and gets mutable access to tranform component
    for (mut tranform, mut player) in &mut characters {
        player.direction = Vec2::new(0.0, 0.0);
        let mut x = 0.0;
        let mut y = 0.0;
 
        if input.pressed(KeyCode::W) {
            y += player.speed * time.delta_seconds();
            player.direction += Vec2::new(0.0, 1.0);
        }
        if input.pressed(KeyCode::S) {
            y -= player.speed * time.delta_seconds();
            player.direction += Vec2::new(0.0, -1.0);
        }
        if input.pressed(KeyCode::A) {
            x -= player.speed * time.delta_seconds();
            player.direction += Vec2::new(-1.0, 0.0);
        }
        if input.pressed(KeyCode::D) {
            x += player.speed * time.delta_seconds();
            player.direction += Vec2::new(1.0, 0.0);
        }

        //kill velocity if next to window edge
        if tranform.translation.x < ((-(WINDOW_WIDTH-PLAYER_X_SIZE))/2.0) && x < 0.0 {
            x = 0.0;
        }
        if tranform.translation.x > (((WINDOW_WIDTH-PLAYER_X_SIZE))/2.0) && x > 0.0 {
            x = 0.0;
        }
        if tranform.translation.y < ((-(WINDOW_HEIGHT-PLAYER_Y_SIZE))/2.0) && y < 0.0 {
            y = 0.0;
        }
        if tranform.translation.y > (((WINDOW_HEIGHT-PLAYER_Y_SIZE))/2.0) && y > 0.0 {
            y = 0.0;
        }  

        tranform.translation.x += x;
        player.position.x += x;
        tranform.translation.y += y;
        player.position.y += y;
    }
}

fn decode_save (
    save_info: String,
) -> Vec<String> 
{
    let mut stats = Vec::new();
    for line in save_info.lines() {
        stats.push(String::from(line));
    };
    stats
}


//THE TEAR SECTION

#[derive(Component)]
pub struct Tear{
    pub direction: Vec2,
}

pub const COOLDOWN_CONST: f32 = 1.0;
pub const TEAR_SPEED: f32 = 200.0;

fn tear_spawn_system(
    mut player: Query<&mut Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    let mut tear_direction = Vec2::new(0.0, 0.0);
    let mut spawn = false;

    if input.pressed(KeyCode::Up){
        tear_direction += Vec2::new(0.0, 1.0);
        spawn = true;
    }
    if input.pressed(KeyCode::Down){
        tear_direction += Vec2::new(0.0, -1.0);
        spawn = true;
    }
    if input.pressed(KeyCode::Left){
        tear_direction += Vec2::new(-1.0, 0.0);
        spawn = true;
    }
    if input.pressed(KeyCode::Right){
        tear_direction += Vec2::new(1.0, 0.0);
        spawn = true;
    }

    for mut player in &mut player {

        if player.tear_cooldown > 0.0 {
            player.tear_cooldown -= COOLDOWN_CONST * time.delta_seconds();
            player.tear_cooldown = player.tear_cooldown.clamp(0.0, 100.0);
            spawn = false;
        }

        tear_direction += player.direction * (PLAYER_SPEED / TEAR_SPEED);

        if tear_direction == Vec2::new(0.0, 0.0){
            spawn = false;
        }
        
        if spawn == true {
            let texture_handle = asset_server.load("sprites/tear.png");

            let tear = Tear{
                direction: tear_direction,
            };

            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PLAYER_X_SIZE, PLAYER_Y_SIZE)),
                    ..default()
                },
                texture: texture_handle,
                transform: Transform {
                    translation: Vec3::new(player.position.x, player.position.y, 0.0),
                    ..default()
                },
                ..default()
            }, tear));

            player.tear_cooldown = COOLDOWN_CONST * 0.2;
        }
    }
}

fn tear_movement_system(
    mut tears: Query<(&mut Transform, &Tear)>,
    time: Res<Time>,
){
    for (mut tranform, tear) in &mut tears {
        tranform.translation.x += tear.direction.x * TEAR_SPEED * time.delta_seconds();
        tranform.translation.y += tear.direction.y * TEAR_SPEED * time.delta_seconds();
    }
}