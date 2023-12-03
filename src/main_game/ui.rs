use bevy::prelude::*;

use crate::{asset_loader::FontAssets, config::hex};

pub struct UiPlugin;

const COLOR_BUTTON_TEXT: &str = "#10141f";
const COLOR_TITLE_LABEL: &str = "#75a743";
const MENU_BUTTON_FONT_SIZE: f32 = 80.;
const TITLE_LABEL_FONT_SIZE: f32 = 120.;

pub const COLOR_BUTTON_BACKGROUND: &str = "#73bed3";
pub const COLOR_BUTTON_BACKGROUND_HOVER: &str = "#4f8fba";
pub const COLOR_BUTTON_BACKGROUND_PRESSED: &str = "#3c5e8b";

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu)
            .add_systems(Update, (button_appearance_update, button_release_listener))
            .add_event::<ButtonReleaseEvent>();
    }
}

#[derive(Component, Default, PartialEq, Reflect)]
pub enum ButtonState {
    #[default]
    Unpressed,
    Pressed,
}

#[derive(Event)]
pub struct ButtonReleaseEvent {
    pub button_type: ButtonType,
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub enum ButtonType {
    #[default]
    Unknown,
    BuyKernel,
}

#[derive(Bundle, Default)]
struct UiButtonBundle {
    button: ButtonBundle,
    b_type: ButtonType,
    state: ButtonState,
}

fn spawn_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            Name::new("MenuRoot"),
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("Buy menu"),
                    NodeBundle {
                        style: Style {
                            // align_self: AlignSelf::FlexEnd,
                            // flex_direction: FlexDirection::Row,
                            margin: UiRect::bottom(Val::Percent(10.)),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder
                        .spawn((
                            Name::new("Buy Kernel Button"),
                            UiButtonBundle {
                                button: ButtonBundle {
                                    style: buy_button_style(),
                                    border_color: BorderColor(Color::BLACK),
                                    background_color: hex(COLOR_BUTTON_BACKGROUND).into(),
                                    ..default()
                                },
                                b_type: ButtonType::BuyKernel,
                                ..default()
                            },
                        ))
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "Buy Kernel",
                                TextStyle {
                                    font: font_assets.default.clone(),
                                    font_size: MENU_BUTTON_FONT_SIZE,
                                    color: hex(COLOR_BUTTON_TEXT),
                                },
                            ));
                        });
                });
        });
}

fn buy_button_style() -> Style {
    Style {
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    }
}

fn button_appearance_update(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut ButtonState,
            &ButtonType,
        ),
        Changed<Interaction>,
    >,
    mut ev_button_released: EventWriter<ButtonReleaseEvent>,
) {
    for (&interaction, mut background_color, mut button_state, &button_type) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *background_color = hex(COLOR_BUTTON_BACKGROUND_PRESSED).into();
                *button_state = ButtonState::Pressed;
            }
            Interaction::Hovered => {
                if *button_state == ButtonState::Pressed {
                    // Released button
                    ev_button_released.send(ButtonReleaseEvent { button_type })
                }
                *background_color = hex(COLOR_BUTTON_BACKGROUND_HOVER).into();
                *button_state = ButtonState::Unpressed;
            }
            Interaction::None => {
                *background_color = hex(COLOR_BUTTON_BACKGROUND).into();
                *button_state = ButtonState::Unpressed;
            }
        }
    }
}

fn button_release_listener() {
    // TODO
}
