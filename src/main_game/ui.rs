use bevy::prelude::*;

use crate::{
    asset_loader::{FontAssets, TextureAssets},
    config::hex,
    main_game::kernel::SpawnKernelEvent,
};

use super::money::BankAccount;

pub struct UiPlugin;

const BOTTOM_BAR_PADDING: Val = Val::Px(4.);
const COLOR_BUTTON_TEXT: &str = "#10141f";
const MENU_BUTTON_FONT_SIZE: f32 = 40.;

// TODO make these greyscale since we're using image buttons
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

fn spawn_menu(
    mut commands: Commands,
    bank_account: Res<BankAccount>,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands
        .spawn((
            Name::new("Bottom Panel"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    margin: UiRect::top(Val::Auto),
                    padding: UiRect::all(BOTTOM_BAR_PADDING),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("Account panel"),
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Name::new("Account balance label"),
                        Label,
                        TextBundle::from_section(
                            format!("{}", bank_account.as_ref()),
                            TextStyle {
                                font: font_assets.default.clone(),
                                // TODO fix style
                                font_size: 80.,
                                color: hex("#ffffff"),
                            },
                        ),
                    ));
                });

            builder
                .spawn((
                    Name::new("Buy menu"),
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
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
                                    image: UiImage {
                                        texture: texture_assets.raw_kernel.clone(),
                                        ..default()
                                    },
                                    ..default()
                                },
                                b_type: ButtonType::BuyKernel,
                                ..default()
                            },
                        ))
                        .with_children(|builder| {
                            // builder.spawn(TextBundle::from_section(
                            //     // TODO remove for the x1 button. Add a x10, x100 etc. ?
                            //     "Buy Kernel",
                            //     TextStyle {
                            //         font: font_assets.default.clone(),
                            //         font_size: MENU_BUTTON_FONT_SIZE,
                            //         color: hex(COLOR_BUTTON_TEXT),
                            //     },
                            // ));
                        });
                });
        });
}

fn buy_button_style() -> Style {
    Style {
        height: Val::Px(64.),
        width: Val::Px(64.),
        border: UiRect::all(Val::Px(2.0)),
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

fn button_release_listener(
    mut ev_button_released: EventReader<ButtonReleaseEvent>,
    mut ev_spawn_kernel: EventWriter<SpawnKernelEvent>,
) {
    for ev in ev_button_released.read() {
        match ev.button_type {
            ButtonType::BuyKernel => {
                ev_spawn_kernel.send(SpawnKernelEvent);
                info!("Buy Kernel pressed");
            }
            ButtonType::Unknown => {
                warn!("Unknown button pressed");
            }
        }
    }
}
