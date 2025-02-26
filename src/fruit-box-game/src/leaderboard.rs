use crate::state::*;
use bevy::{color::palettes::tailwind::YELLOW_400, prelude::*};

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Leaderboard), setup)
            .add_systems(OnExit(GameState::Leaderboard), tear_down);
    }
}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct Score {
    pub id: usize,
    pub score: usize,
}

#[derive(Component)]
struct Leaderboard;

fn setup(mut commands: Commands, scores: Query<&Score>) {
    let new_score = *scores.iter().max_by_key(|s| s.id).unwrap();
    let top_scores: Vec<Score> = {
        let mut top_scores = scores.iter().map(|s| *s).collect::<Vec<Score>>();
        top_scores.sort_by(|a, b| a.score.cmp(&b.score).reverse());
        top_scores.into_iter().take(5).collect()
    };
    let is_new_score_top_score = top_scores.contains(&new_score);

    commands
        .spawn((
            Leaderboard,
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Leaderboard"),
                TextFont {
                    font_size: 56.,
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Rank\tScore"),
                TextFont {
                    font_size: 32.,
                    ..default()
                },
            ));
            for (index, top_score) in top_scores.iter().enumerate() {
                let text_color = if top_score == &new_score {
                    Color::from(YELLOW_400)
                } else {
                    Color::WHITE
                };
                let rank = index + 1;
                parent.spawn((
                    Text::new(format!("{rank}\t{}", top_score.score)),
                    TextColor(text_color),
                    TextFont {
                        font_size: 32.,
                        ..default()
                    },
                ));
            }
            if !is_new_score_top_score {
                parent.spawn((
                    Text::new(format!("Score\t{}", new_score.score)),
                    TextColor(Color::from(YELLOW_400)),
                    TextFont {
                        font_size: 32.,
                        ..default()
                    },
                ));
            }
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
        });
}

fn tear_down(mut commands: Commands, leaderboard: Single<Entity, With<Leaderboard>>) {
    commands.entity(*leaderboard).despawn_recursive();
}
