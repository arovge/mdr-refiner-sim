use crate::state::*;
use bevy::{ecs::spawn::SpawnWith, prelude::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(OnExit(GameState::MainMenu), tear_down);
    }
}

#[derive(Component)]
struct Menu;

fn setup(mut commands: Commands) {
    commands.spawn((
        Menu,
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(50.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..default()
        },
        Children::spawn((
            Spawn((
                Text::new("MDR Simulator"),
                TextFont {
                    font_size: 56.,
                    ..default()
                },
            )),
            Spawn((
                Text::new("Welcome Refiner"),
                TextFont {
                    font_size: 48.,
                    ..default()
                },
            )),
            SpawnWith(|parent: &mut ChildSpawner| {
                parent
                    .spawn((
                        Text::new("Play"),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 48.,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        Node {
                            padding: UiRect::horizontal(Val::Px(100.)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(12.)),
                        BackgroundColor(Color::WHITE),
                    ))
                    .observe(play);
            }),
        )),
    ));
}

fn tear_down(mut commands: Commands, text: Single<Entity, With<Menu>>) {
    commands.entity(*text).despawn();
}
