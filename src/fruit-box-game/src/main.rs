use bevy::{prelude::*, window::WindowResolution};

const ROWS: usize = 10;
const COLS: usize = 17;
const SCALE: f32 = 100.;
const HEIGHT: f32 = ROWS as f32 * SCALE;
const WIDTH: f32 = COLS as f32 * SCALE;

struct Cell(u32);

#[derive(Resource)]
struct Grid(Vec<Vec<Cell>>);

impl Grid {
    pub fn new() -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for x in 0..COLS {
            grid.push(Vec::new());
            for _ in 0..ROWS {
                let val = rand::random_range(1..=9);
                grid[x].push(Cell(val));
            }
        }
        Grid(grid)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Grid::new())
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>,
) {
    commands.spawn(Camera2d);

    let square = Rectangle::new(SCALE, SCALE);

    for x in 0..COLS {
        for y in 0..ROWS {
            let color = if (x + y) % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };
            commands.spawn((
                Mesh2d(meshes.add(square)),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(
                    (x as f32 * SCALE) - (WIDTH / 2.) + (SCALE / 2.),
                    (y as f32 * SCALE) - (HEIGHT / 2.) + (SCALE / 2.),
                    0.,
                ),
            ));

            let text_color = if (x + y) % 2 == 0 {
                Color::BLACK
            } else {
                Color::WHITE
            };
            commands.spawn((
                Text2d(grid.0[x][y].0.to_string()),
                TextColor(text_color),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(
                    (x as f32 * SCALE) - (WIDTH / 2.) + (SCALE / 2.),
                    (y as f32 * SCALE) - (HEIGHT / 2.) + (SCALE / 2.),
                    1.,
                ),
            ));
        }
    }
}
