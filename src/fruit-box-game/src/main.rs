use std::time::Duration;

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
const TARGET_SUM: usize = 10;

#[derive(Clone, Copy)]
enum Status {
    Default,
    Selected,
    Hidden,
}

#[derive(Component, Clone, Copy)]
struct Cell {
    col: usize,
    row: usize,
    value: usize,
    status: Status,
}

#[derive(Default, Clone, Copy)]
struct Position {
    col: usize,
    row: usize,
}

#[derive(Clone, Default)]
enum DragGesture {
    #[default]
    NotDragging,
    Dragging {
        start: Position,
    },
    Ended,
}

impl DragGesture {
    fn start(&self) -> Option<Position> {
        match self {
            DragGesture::Dragging { start } => Some(start.clone()),
            _ => None,
        }
    }
}

#[derive(Resource, Default)]
struct DragState(DragGesture);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct CountdownTimer(Timer);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DragState::default())
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
) {
    commands.spawn(Camera2d);

    let square = Rectangle::new(SCALE, SCALE);
    let cell_color = materials.add(Color::WHITE);
    let text_color = Color::BLACK;
    let grid = build_cells();

    for x in 0..COLS {
        for y in 0..ROWS {
            let cell = grid[x][y].clone();
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
                .observe(drag_start)
                .observe(drag_over)
                .observe(drag_end);
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
        Text::new(format_duration(Duration::from_secs_f32(
            GAME_DURACTION_SECS,
        ))),
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

fn drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    query: Query<&Cell>,
    mut drag_state: ResMut<DragState>,
) {
    let cell = query.get(trigger.entity()).unwrap();
    drag_state.0 = DragGesture::Dragging {
        start: Position {
            col: cell.col,
            row: cell.row,
        },
    };
}

fn drag_over(
    trigger: Trigger<Pointer<DragOver>>,
    mut drag_state: ResMut<DragState>,
    mut cells: Query<&mut Cell>,
) {
    let cell = cells.get(trigger.entity()).unwrap();
    let drag_start = drag_state.0.start();
    drag_state.0 = DragGesture::Dragging {
        start: drag_start.unwrap_or(Position::default()),
    };
    let Some(drag_start) = drag_start else { return };

    let col_range = drag_start.col.min(cell.col)..=drag_start.col.max(cell.col);
    let row_range = drag_start.row.min(cell.row)..=drag_start.row.max(cell.row);

    for mut cell in cells.iter_mut() {
        if matches!(cell.status, Status::Hidden) {
            continue;
        }
        cell.status = if col_range.contains(&cell.col) && row_range.contains(&cell.row) {
            Status::Selected
        } else {
            Status::Default
        };
    }
}

fn drag_end(_trigger: Trigger<Pointer<DragEnd>>, mut drag_state: ResMut<DragState>) {
    drag_state.0 = DragGesture::Ended;
}

fn update_cells(
    mut drag_state: ResMut<DragState>,
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
        .filter(|(cell, _material, _children)| matches!(cell.status, Status::Selected))
        .map(|(cell, _material, _children)| cell.value)
        .sum::<usize>();

    let selected_color = if selected_total == TARGET_SUM {
        summed_selected_color
    } else {
        selected_color
    };

    let drag_ended = matches!(drag_state.0, DragGesture::Ended);
    if drag_ended {
        drag_state.0 = DragGesture::NotDragging;
    }

    for (mut cell, mut material, children) in cells.iter_mut() {
        if drag_ended && matches!(cell.status, Status::Selected) {
            cell.status = if selected_total == TARGET_SUM {
                Status::Hidden
            } else {
                Status::Default
            };
        }
        match cell.status {
            Status::Default => {
                material.0 = cell_color.clone();
            }
            Status::Selected => {
                material.0 = selected_color.clone();
            }
            Status::Hidden => {
                material.0 = hidden_color.clone();
                for child in children {
                    let mut text = cell_text.get_mut(*child).unwrap();
                    text.0 = String::new();
                }
            }
        }
    }
}

fn update_score(cells: Query<&mut Cell>, mut score_text: Single<&mut Text, With<ScoreText>>) {
    let score = cells
        .iter()
        .filter(|cell| matches!(cell.status, Status::Hidden))
        .count();

    score_text.0 = format!("Score: {score}");
}

fn update_timer(
    time: Res<Time>,
    mut timer: Single<&mut CountdownTimer>,
    mut countdown_text: Single<&mut Text, With<CountdownText>>,
) {
    timer.0.tick(time.delta());
    countdown_text.0 = format_duration(timer.0.remaining());
}

fn format_duration(duration: Duration) -> String {
    format!(
        "{:0>1}:{:0>2}",
        duration.as_secs() / 60,
        duration.as_secs() % 60
    )
}

fn build_cells() -> Vec<Vec<Cell>> {
    let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(COLS);
    for col in 0..COLS {
        let mut rows = Vec::with_capacity(ROWS);
        for row in 0..ROWS {
            let value = rand::random_range(1..=9);
            let cell = Cell {
                col,
                row,
                value,
                status: Status::Default,
            };
            rows.push(cell);
        }

        cells.push(rows);
    }
    cells
}
