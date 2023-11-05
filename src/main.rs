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
            translation: Vec3::new(0.0, (WINDOW_HEIGHT/2.0)-(PLAYER_Y_SIZE/2.0), 0.0),
            ..default()
        },
        ..default()
    }, player));
}

fn movement(
    //calls only entities with both Transform and PLayer components
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    //loops over all entities that match query and gets mutable access to tranform component
    for (mut tranform, player) in &mut characters {
        let mut x = 0.0;
        let mut y = 0.0;

        if input.pressed(KeyCode::Up) {
            y += player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::Down) {
            y -= player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::Left) {
            x -= player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::Right) {
            x += player.speed * time.delta_seconds();
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
        tranform.translation.y += y;
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