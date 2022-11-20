use bevy::ecs::query;
use rand::prelude::SliceRandom;
use rand::distributions::{Distribution, Uniform};
use core::time::Duration;

use bevy::{prelude::*, audio, time};
use bevy::time::FixedTimestep;
use bevy::app::AppExit;
use rand::prelude::random;
use bevy::ecs::system::Query;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const MOVEMENT_DURATION: f32 = 0.3;

struct GrowthEvent;
struct GameOverEvent;


#[derive(Component)]
struct SnakeHead{
    direction: Direction,
}

#[derive(Component, Resource)]
struct DirectionChangeTimer{
    //timer that prevents the snake from turning too fast
    timer: Timer,
}

#[derive(Component, Resource)]
struct SnakeHeadDirection{
    direction: Direction,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
struct SnakeSegments(Vec<Entity>);


#[derive(Component)]
struct Food;

#[derive(Component)]
struct GrassTile;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

#[derive(Component, Default, Resource)]
struct FreePosition(Option<Position>);

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

fn start_snake_turn_prevention_timer(mut commands: Commands) {
    commands.insert_resource(DirectionChangeTimer {
        timer: Timer::from_seconds(MOVEMENT_DURATION, TimerMode::Repeating),
    });
}


fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_with_settings(
        asset_server.load("background_music.ogg"),
        PlaybackSettings::LOOP.with_volume(0.75),
    );
}


fn animate_snake_head(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        Option<&SnakeHead>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, snakehead) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            
            if let Some(snakehead) = snakehead {
                match snakehead.direction {
                    Direction::Down => sprite.index = (sprite.index % 4),
                    Direction::Up => sprite.index = (sprite.index % 4) + 4,
                    Direction::Left => sprite.index = (sprite.index % 4) + 8,
                    Direction::Right => sprite.index = (sprite.index % 4) + 12,
                }
            }
        }
    }
}


fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32 / 24.0,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32 / 24.0,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle_1 = asset_server.load("grass1_72x24.png");
    let texture_atlas_1 =
        TextureAtlas::from_grid(texture_handle_1, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_1 = texture_atlases.add(texture_atlas_1);

    let texture_handle_2 = asset_server.load("grass1_72x24.png");
    let texture_atlas_2 =
        TextureAtlas::from_grid(texture_handle_2, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_2 = texture_atlases.add(texture_atlas_2);

    let texture_handle_3 = asset_server.load("grass3_72x24.png");
    let texture_atlas_3 =
        TextureAtlas::from_grid(texture_handle_3, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_3 = texture_atlases.add(texture_atlas_3);

    let texture_handle_4 = asset_server.load("grass6_72x24.png");
    let texture_atlas_4 =
        TextureAtlas::from_grid(texture_handle_4, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_4 = texture_atlases.add(texture_atlas_4);

    let texture_handle_5 = asset_server.load("grass5_72x24.png");
    let texture_atlas_5 =
        TextureAtlas::from_grid(texture_handle_5, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_5 = texture_atlases.add(texture_atlas_5);

    let texture_handle_6 = asset_server.load("grass6_72x24.png");
    let texture_atlas_6 =
        TextureAtlas::from_grid(texture_handle_6, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_6 = texture_atlases.add(texture_atlas_6);

    let texture_handle_7 = asset_server.load("grass7_72x24.png");
    let texture_atlas_7 =
        TextureAtlas::from_grid(texture_handle_7, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_7 = texture_atlases.add(texture_atlas_7);

    let texture_handle_8 = asset_server.load("grass8_72x24.png");
    let texture_atlas_8 =
        TextureAtlas::from_grid(texture_handle_8, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle_8 = texture_atlases.add(texture_atlas_8);

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..8);
    let mut throw;
    let mut texture_atlas_handle;
    for j in 0..ARENA_WIDTH {
        for i in 0..ARENA_HEIGHT {
            throw = die.sample(&mut rng);
            match throw{
                1 => texture_atlas_handle =  texture_atlas_handle_1.clone(),
                2 => texture_atlas_handle =  texture_atlas_handle_2.clone(),
                3 => texture_atlas_handle =  texture_atlas_handle_3.clone(),
                4 => texture_atlas_handle =  texture_atlas_handle_4.clone(),
                5 => texture_atlas_handle =  texture_atlas_handle_5.clone(),
                6 => texture_atlas_handle =  texture_atlas_handle_6.clone(),
                7 => texture_atlas_handle =  texture_atlas_handle_7.clone(),
                8 => texture_atlas_handle =  texture_atlas_handle_8.clone(),
                _ => texture_atlas_handle =  texture_atlas_handle_1.clone(),
            };
            commands
                .spawn((SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_xyz(
                        j as f32 * 24.0,
                        i as f32 * 24.0,
                        0.0,
                    ),
                    sprite: TextureAtlasSprite::new(0),
                    ..default()
                },
                AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
                ))
                .insert(Position { x:j as i32, y:i as i32})
                .insert(Size::square(1.0));

        }
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,)
{
    let texture_handle = asset_server.load("character96x96.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 4, None, None);
    let texture_atlas_handle =  texture_atlases.add(texture_atlas);

    *segments = SnakeSegments(vec![
        commands
            .spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(
                    3 as f32 * 24.0,
                    3 as f32 * 24.0,
                    5.0,
                ),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.075, TimerMode::Repeating)),
            ))
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(1.0))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }, texture_atlases, asset_server),
    ]);
}

fn spawn_segment(
    mut commands: Commands,
    position: Position,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) -> Entity {

    let texture_handle = asset_server.load("rock24x24.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn((SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    ..default()
                },
                AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
                ))
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(1.0))
        .id()
}
fn spawn_food(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
){
    let texture_handle = asset_server.load("floatingrock72x24.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        ))
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(1.0));
        
}

fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
    snake_head_direction: Res<SnakeHeadDirection>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
  
        if dir != snake_head_direction.direction.opposite(){
            head.direction = dir;
        }

    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut head_dir : ResMut<SnakeHeadDirection>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head_dir.direction = head.direction;
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
        || head_pos.y < 0
        || head_pos.x as u32 >= ARENA_WIDTH
        || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if growth_reader.iter().next().is_some() {
        audio.play(asset_server.load("eat_sound.ogg"));
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap(), texture_atlases, asset_server));
    }
}

fn empty_position(
    occupied_positions: Query<&Position, With<SnakeSegment>>,
    mut free_position: ResMut<FreePosition>,
){
    let mut all_positions = Vec::new();
    for i in 0..ARENA_HEIGHT{
        for j in 0..ARENA_WIDTH{
            all_positions.push(Position{y:i as i32, x:j as i32})
        }
    }

    let mut empty_positions = all_positions;
    for pos in occupied_positions.iter(){
        empty_positions.retain(|x| x != pos);
    }
    
    let result = empty_positions.choose(&mut rand::thread_rng()).unwrap();
    free_position.0 = Some(*result);
}

fn food_spawner(
    mut commands: Commands,
    free_position: Res<FreePosition>,
    mut growth_event_listener: EventReader<GrowthEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("floatingrock72x24.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
    if growth_event_listener.iter().next().is_some(){
        commands
        .spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        ))
        .insert(Food)
        // .insert(Position {
        //     x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        //     y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        // })
        .insert(free_position.0.unwrap())
        .insert(Size::square(1.0));
    }   
}

fn exit_system(mut exit: EventWriter<AppExit>, mut gameover: EventReader<GameOverEvent>) {
    if gameover.iter().next().is_some(){
        exit.send(AppExit);
    }
    
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(SnakeHeadDirection{direction:Direction::Up})
    .insert_resource(SnakeSegments::default())
    .insert_resource(LastTailPosition::default())
    .insert_resource(FreePosition::default())
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_tiles)
    .add_startup_system(play_music)
    .add_startup_system(spawn_snake.after(spawn_tiles))
    .add_startup_system(spawn_food)
    .add_startup_system(start_snake_turn_prevention_timer)
    .add_system(exit_system)
    .add_system(snake_movement_input.after(start_snake_turn_prevention_timer))
    .add_system(animate_snake_head)
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.30))
            .with_system(snake_movement)
            .with_system(snake_eating.after(snake_movement))
            .with_system(snake_growth.after(snake_eating)),
    )

    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(position_translation)
            .with_system(size_scaling),
    )
    .add_system_set(
        SystemSet::new()
            .with_system(empty_position)
            .with_system(food_spawner.after(empty_position)),
    )
    .add_event::<GrowthEvent>()
    .add_event::<GameOverEvent>()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Carriage Snake!".to_string(), 
            width: 700.0,                 
            height: 700.0,
          ..default()
        },
        ..default()
      }, 
    )
    .set(ImagePlugin::default_nearest()))
    .run();

}

