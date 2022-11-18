use rand::prelude::SliceRandom;
use std::env;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::app::AppExit;
use rand::prelude::random;
use bevy::ecs::system::Query;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;


struct GrowthEvent;
struct GameOverEvent;


#[derive(Component)]
struct SnakeHead{
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

fn animate_snake_head(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
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

fn size_scaling_background(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
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

fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let grass_handle = asset_server.load("iarba_64x64.png");
    let grass_handle = asset_server.load("1.png");
    for j in 0..ARENA_WIDTH {
        for i in 0..ARENA_HEIGHT {
            commands
                .spawn(SpriteBundle {
                    texture:grass_handle.clone(),

                    ..Default::default()
                })
                .insert(Position { x:j as i32, y:i as i32})
                .insert(Size::square(1.0));

        }
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,) {
    
    let texture_handle = asset_server.load("test.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    *segments = SnakeSegments(vec![
        commands
            .spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ))
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(1.0))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
        
        
    ]);
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(24.0))
        .id()
}
fn spawn_food(mut commands: Commands){
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(24.0));
        
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
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
        if dir != head.direction.opposite() {
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
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
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
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
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
) {
    if growth_event_listener.iter().next().is_some(){
        commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        // .insert(Position {
        //     x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        //     y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        // })
        .insert(free_position.0.unwrap())
        .insert(Size::square(24.0));
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
    .insert_resource(SnakeSegments::default())
    .insert_resource(LastTailPosition::default())
    .insert_resource(FreePosition::default())
    .add_startup_system(setup_camera)
    //.add_startup_system(spawn_tiles)
    .add_startup_system(spawn_snake)
    .add_startup_system(spawn_food)
    .add_system(exit_system)
    .add_system(snake_movement_input.before(snake_movement))
    .add_system(animate_snake_head)
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.150))
            .with_system(snake_movement)
            .with_system(snake_eating.after(snake_movement))
            .with_system(snake_growth.after(snake_eating)),
    )
    // .add_startup_system_set_to_stage(
    //     StartupStage::PostStartup,
    //     SystemSet::new()
    //         //.with_system(position_translation)
    //         .with_system(size_scaling_background),
    // )
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(position_translation)
            .with_system(size_scaling),
    )
    .add_system_set(
        SystemSet::new()
            //.with_run_criteria(FixedTimestep::step(1.0))
            .with_system(empty_position)
            .with_system(food_spawner.after(empty_position)),
    )
    .add_event::<GrowthEvent>()
    .add_event::<GameOverEvent>()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Carriage Snake!".to_string(), 
            width: 500.0,                 
            height: 500.0,
          ..default()
        },
        ..default()
      }, 
    )
    .set(ImagePlugin::default_nearest()))
    .run();

}

