use bevy::{
    color::palettes::tailwind::{RED_300, RED_400},
    prelude::*,
    window::WindowResolution,
};

const ROWS: usize = 10;
const COLS: usize = 17;
const SCALE: f32 = 100.;
const HEIGHT: f32 = ROWS as f32 * SCALE;
const WIDTH: f32 = (COLS + 3) as f32 * SCALE;
const GAME_DURACTION_SECS: f32 = 120.;

#[derive(Clone, Copy)]
enum CellStatus {
    Selected,
    Idle,
    Hidden,
}

#[derive(Component, Clone, Copy)]
struct Cell {
    col: usize,
    row: usize,
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
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(COLS);
        for col in 0..COLS {
            let mut rows = Vec::with_capacity(ROWS);
            for row in 0..ROWS {
                let value = rand::random_range(1..=9);
                let cell = Cell {
                    col,
                    row,
                    value,
                    status: CellStatus::Idle,
                };
                rows.push(cell);
            }

            grid.push(rows);
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
    let cell = query.get(trigger.entity()).unwrap();
    if matches!(cell.status, CellStatus::Hidden) {
        return;
    }

    // Selected cells must be in the same axis
    let is_axis_aligned = query
        .iter()
        .filter(|c| matches!(c.status, CellStatus::Selected))
        .all(|c| c.row == cell.row || c.col == cell.col);

    if !is_axis_aligned {
        return;
    }

    let is_first_selected_cell = query
        .iter()
        .all(|c| !matches!(c.status, CellStatus::Selected));

    // Selected cells must be connected to another selected cell,
    // or be the first selected cell
    if !is_first_selected_cell {
        let is_connected = query
            .iter()
            .filter(|c| matches!(c.status, CellStatus::Selected))
            .any(|c| {
                (c.row + 1 == cell.row && c.col == cell.col)
                    || (c.row - 1 == cell.row && c.col == cell.col)
                    || (c.row == cell.row && c.col + 1 == cell.col)
                    || (c.row == cell.row && c.col - 1 == cell.col)
            });

        if !is_connected {
            return;
        }
    }

    let mut cell = query.get_mut(trigger.entity()).unwrap();

    cell.status = CellStatus::Selected;
}

fn end_drag(_trigger: Trigger<Pointer<DragEnd>>, mut query: Query<&mut Cell>) {
    let mut selected_cells = query
        .iter_mut()
        .filter(|cell| matches!(cell.status, CellStatus::Selected))
        .collect::<Vec<_>>();
    let sum = selected_cells.iter().map(|cell| cell.value).sum::<usize>();

    let status = if sum == 10 {
        CellStatus::Hidden
    } else {
        CellStatus::Idle
    };

    for cell in selected_cells.iter_mut() {
        cell.status = status.clone();
    }
}

fn update_cells(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cells: Query<(&mut Cell, &mut MeshMaterial2d<ColorMaterial>, &Children)>,
    mut cell_text: Query<&mut Text2d>,
) {
    let cell_color = materials.add(Color::WHITE);
    let hidden_color = materials.add(Color::BLACK);
    let selected_color = materials.add(Color::from(RED_300));
    let summed_selected_color = materials.add(Color::from(RED_400));

    let selected_total = cells
        .iter()
        .filter(|(cell, _material, _children)| matches!(cell.status, CellStatus::Selected))
        .map(|(cell, _material, _children)| cell.value)
        .sum::<usize>();

    let selected_color = if selected_total == 10 {
        summed_selected_color
    } else {
        selected_color
    };

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
