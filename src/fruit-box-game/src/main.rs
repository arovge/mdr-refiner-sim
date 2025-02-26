use bevy::{color::palettes::tailwind::CYAN_300, prelude::*, window::WindowResolution};

const ROWS: usize = 10;
const COLS: usize = 17;
const SCALE: f32 = 100.;
const HEIGHT: f32 = ROWS as f32 * SCALE;
const WIDTH: f32 = (COLS + 3) as f32 * SCALE;
const GAME_DURACTION_SECS: f32 = 120.;

#[derive(Clone)]
enum CellStatus {
    Selected,
    Idle,
    Hidden,
}

#[derive(Component, Clone)]
struct Cell {
    value: usize,
    status: CellStatus,
}

#[derive(Resource)]
struct Grid(Vec<Vec<Cell>>);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct CountdownTimer(Timer);

impl Grid {
    pub fn new() -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for col in 0..COLS {
            grid.push(Vec::new());

            for _ in 0..ROWS {
                let value = rand::random_range(1..=9);
                let cell = Cell {
                    value,
                    status: CellStatus::Idle,
                };
                grid[col].push(cell);
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
        .add_systems(Update, ((update_cells, update_score).chain(), update_timer))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WIDTH, HEIGHT)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
        ))
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
    let cell_color = materials.add(Color::WHITE);
    let text_color = Color::BLACK;

    for x in 0..COLS {
        for y in 0..ROWS {
            let cell = grid.0[x][y].clone();
            commands
                .spawn((
                    cell.clone(),
                    Mesh2d(meshes.add(square)),
                    MeshMaterial2d(cell_color.clone()),
                    Transform::from_xyz(
                        (x as f32 * SCALE) - (WIDTH / 2.) + (SCALE / 2.),
                        (y as f32 * SCALE) - (HEIGHT / 2.) + (SCALE / 2.),
                        0.,
                    ),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2d(cell.clone().value.to_string()),
                        TextColor(text_color),
                        TextFont {
                            font_size: 32.,
                            ..default()
                        },
                    ));
                })
                .observe(drag_over_cell::<Pointer<DragEnter>>)
                .observe(drag_over_cell::<Pointer<Drag>>)
                .observe(end_drag);
        }
    }

    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 32.,
            ..default()
        },
        TextLayout {
            justify: JustifyText::Right,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(15.),
            right: Val::Px(15.),
            ..default()
        },
    ));

    commands.spawn((
        CountdownText,
        Text::new("2:00"),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 32.,
            ..default()
        },
        TextLayout {
            justify: JustifyText::Right,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.),
            right: Val::Px(15.),
            ..default()
        },
    ));
    commands.spawn(CountdownTimer(Timer::from_seconds(
        GAME_DURACTION_SECS,
        TimerMode::Once,
    )));
}

fn drag_over_cell<E>(trigger: Trigger<E>, mut query: Query<&mut Cell>) {
    let mut cell = query.get_mut(trigger.entity()).unwrap();
    if !matches!(cell.status, CellStatus::Hidden) {
        cell.status = CellStatus::Selected;
    }
}

fn end_drag(_trigger: Trigger<Pointer<DragEnd>>, mut query: Query<&mut Cell>) {
    let mut selected_cells = query
        .iter_mut()
        .filter(|cell| matches!(cell.status, CellStatus::Selected))
        .collect::<Vec<_>>();
    let sum = selected_cells.iter().map(|cell| cell.value).sum::<usize>();

    if sum == 10 {
        for cell in selected_cells.iter_mut() {
            cell.status = CellStatus::Hidden;
        }
    } else {
        for cell in selected_cells.iter_mut() {
            cell.status = CellStatus::Idle;
        }
    }
}

fn update_cells(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cells: Query<(&mut Cell, &mut MeshMaterial2d<ColorMaterial>, &Children)>,
    mut cell_text: Query<&mut Text2d>,
) {
    let cell_color = materials.add(Color::WHITE);
    let hidden_color = materials.add(Color::BLACK);
    let selected_color = materials.add(Color::from(CYAN_300));

    for (cell, mut material, children) in cells.iter_mut() {
        match cell.status {
            CellStatus::Selected => {
                material.0 = selected_color.clone();
            }
            CellStatus::Hidden => {
                material.0 = hidden_color.clone();
                for child in children {
                    let mut text = cell_text.get_mut(*child).unwrap();
                    text.0 = String::new();
                }
            }
            CellStatus::Idle => {
                material.0 = cell_color.clone();
            }
        }
    }
}

fn update_score(cells: Query<&mut Cell>, mut score_text: Single<&mut Text, With<ScoreText>>) {
    let score = cells
        .iter()
        .filter(|cell| matches!(cell.status, CellStatus::Hidden))
        .count();

    score_text.0 = format!("Score: {score}");
}

fn update_timer(
    time: Res<Time>,
    mut timer: Single<&mut CountdownTimer>,
    mut countdown_text: Single<&mut Text, With<CountdownText>>,
) {
    timer.0.tick(time.delta());

    countdown_text.0 = format!(
        "{:0>1}:{:0>2}",
        timer.0.remaining().as_secs() / 60,
        timer.0.remaining().as_secs() % 60
    );
}
