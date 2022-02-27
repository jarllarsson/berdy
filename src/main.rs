mod player;
mod enemy;
mod level;

use std::f32::consts::PI;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use level::LevelPlugin;
use std::path::Path;
use bevy::render::texture::ImageType;

const SPRITE_DIR: &str = "assets";
const PLAYER_SPRITE: &str = "test.png";
const BULLET_SPRITE: &str = "test.png";
const ENEMY_SPRITE: &str = "test.png";
const LEVEL_SPRITE: &str = "level_test.png";
const TIME_STEP: f32 = 1. / 60.;

const TO_DEG: f32 = 180. / PI;
const TO_RAD: f32 = PI / 180.;

// Paradigms:
// Entity, Component, System, Resource, Asset(disk)

// Resources

// Sprite images (asset and size)
pub struct SpriteImages {
    player: (Handle<Image>, Vec2),
    bullet: (Handle<Image>, Vec2),
    enemy:  (Handle<Image>, Vec2),
    level:  (Handle<Image>, Vec2),
}
struct WinSize {
    w: f32,
    h: f32
}
struct ActiveEnemies(u32);

// Components 
#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerReadyFire(bool);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Level;

#[derive(Component)]
struct Speed(f32);
impl Default for Speed
{
    fn default() -> Self {
        Self(500.)
    }
}

// App
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Rust Game Test".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(LevelPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    // window.set_position(IVec2::new(2300, 400)); // PC
    window.set_position(IVec2::new(900, 300)); // Laptop

    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Materials
    commands.insert_resource(SpriteImages{
        player: load_image(&mut images, PLAYER_SPRITE),
        bullet: load_image(&mut images, BULLET_SPRITE),
        enemy:  load_image(&mut images, ENEMY_SPRITE),
        level:  load_image(&mut images, LEVEL_SPRITE),
    });

    // Misc resources
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

}

// Note - With bevy v0.6, load images directly and synchronously to capture size
//        See https://github.com/bevyengine/bevy/pull/3696
fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
	let path = Path::new(SPRITE_DIR).join(path);
	let bytes = std::fs::read(&path).expect(&format!("Cannot find {}", path.display()));
	let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
	let size = image.texture_descriptor.size;
	let size = Vec2::new(size.width as f32, size.height as f32);
	let image_handle = images.add(image);
	(image_handle, size)
}
