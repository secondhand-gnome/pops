use bevy::{a11y::accesskit::TextAlign, prelude::*};
use bevy_rapier2d::na::balancing::balance_parlett_reinsch;

use crate::{
    asset_loader::{FontAssets, TextureAssets},
    config::hex,
    main_game::kernel::KernelSpawnEvent,
};

use super::{
    bank_account::BankAccount,
    economy::PriceChecker,
    kernel::{KernelPurchaseEvent, PopCounter},
};

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
            .add_systems(
                Update,
                (
                    button_appearance_update,
                    button_release_listener,
                    update_account_balance,
                    update_pop_count,
                ),
            )
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

    /// Button to buy a quantity of raw kernels
    BuyKernel(u64),

    /// Button to sell a quantity of popped kernels
    SellPopcorn(u64),
}

#[derive(Bundle, Default)]
struct UiButtonBundle {
    button: ButtonBundle,
    b_type: ButtonType,
    state: ButtonState,
}

#[derive(Component)]
struct AccountBalanceLabel;

#[derive(Component)]
struct PopCountLabel;

fn spawn_menu(
    mut commands: Commands,
    bank_account: Res<BankAccount>,
    price_checker: Res<PriceChecker>,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands
        .spawn((
            Name::new("Top panel"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    margin: UiRect::bottom(Val::Auto),
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
                        AccountBalanceLabel,
                        Label,
                        TextBundle::from_section(
                            bank_account.to_string(),
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
                    Name::new("Pop counter panel"),
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
                        Name::new("Pop Count Label"),
                        PopCountLabel,
                        Label,
                        TextBundle::from_sections([
                            TextSection {
                                value: "Pops: ".to_string(),
                                style: TextStyle {
                                    font: font_assets.default.clone(),
                                    // TODO fix style
                                    font_size: 80.,
                                    color: hex("#ffffff"),
                                },
                            },
                            TextSection {
                                value: "0".to_string(),
                                style: TextStyle {
                                    font: font_assets.default.clone(),
                                    // TODO fix style
                                    font_size: 80.,
                                    color: hex("#ffffff"),
                                },
                            },
                        ]),
                    ));
                });
        });

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
                    Name::new("Buy panel"),
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Name::new("Buy label"),
                        TextBundle::from_section(
                            "Buy",
                            TextStyle {
                                font: font_assets.default.clone(),
                                font_size: 28.,
                                color: hex(COLOR_BUTTON_TEXT),
                            },
                        ),
                    ));
                    builder
                        .spawn((
                            Name::new("Buy buttons"),
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
                            let unlocked_kernel_purchase_options = vec![1, 10];
                            for quantity in unlocked_kernel_purchase_options {
                                builder
                                    .spawn(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|builder| {
                                        builder
                                            .spawn((
                                                Name::new(format!(
                                                    "Buy Kernel Button - {quantity}"
                                                )),
                                                UiButtonBundle {
                                                    button: ButtonBundle {
                                                        style: buy_button_style(),
                                                        border_color: BorderColor(Color::BLACK),
                                                        background_color: hex(
                                                            COLOR_BUTTON_BACKGROUND,
                                                        )
                                                        .into(),
                                                        image: UiImage {
                                                            texture: texture_assets
                                                                .raw_kernel
                                                                .clone(),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    b_type: ButtonType::BuyKernel(quantity),
                                                    ..default()
                                                },
                                            ))
                                            .with_children(|builder| {
                                                builder.spawn((
                                                    Name::new(format!(
                                                        "Buy Kernel quantity label - {quantity}"
                                                    )),
                                                    TextBundle::from_section(
                                                        format!("{}", quantity),
                                                        TextStyle {
                                                            font: font_assets.default.clone(),
                                                            font_size: 20.,
                                                            color: hex(COLOR_BUTTON_TEXT),
                                                        },
                                                    ),
                                                ));
                                            });

                                        let price = price_checker.raw_kernels(quantity);
                                        builder.spawn((
                                            Name::new(format!(
                                                "Buy Kernel price label - {quantity}"
                                            )),
                                            TextBundle::from_section(
                                                format!("${:.2}", price),
                                                TextStyle {
                                                    font: font_assets.default.clone(),
                                                    font_size: 20.,
                                                    color: hex(COLOR_BUTTON_TEXT),
                                                },
                                            ),
                                        ));
                                    });
                            }
                        });
                });

            builder
                .spawn((
                    Name::new("Sell panel"),
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Name::new("Sell label"),
                        TextBundle::from_section(
                            "Sell",
                            TextStyle {
                                font: font_assets.default.clone(),
                                font_size: 28.,
                                color: hex(COLOR_BUTTON_TEXT),
                            },
                        ),
                    ));
                    builder
                        .spawn((
                            Name::new("Sell buttons"),
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
                            let unlocked_sell_options = vec![1, 10];
                            for quantity in unlocked_sell_options {
                                builder
                                    .spawn(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|builder| {
                                        builder
                                            .spawn((
                                                Name::new(format!(
                                                    "Sell Popcorn Button - {quantity}"
                                                )),
                                                UiButtonBundle {
                                                    button: ButtonBundle {
                                                        style: buy_button_style(),
                                                        border_color: BorderColor(Color::BLACK),
                                                        background_color: hex(
                                                            COLOR_BUTTON_BACKGROUND,
                                                        )
                                                        .into(),
                                                        image: UiImage {
                                                            texture: texture_assets
                                                                .raw_kernel
                                                                .clone(),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    b_type: ButtonType::SellPopcorn(quantity),
                                                    ..default()
                                                },
                                            ))
                                            .with_children(|builder| {
                                                builder.spawn((
                                                    Name::new(format!(
                                                        "Sell popcorn quantity label - {quantity}"
                                                    )),
                                                    TextBundle::from_section(
                                                        format!("{}", quantity),
                                                        TextStyle {
                                                            font: font_assets.default.clone(),
                                                            font_size: 20.,
                                                            color: hex(COLOR_BUTTON_TEXT),
                                                        },
                                                    ),
                                                ));
                                            });

                                        let price = price_checker.raw_kernels(quantity);
                                        builder.spawn((
                                            Name::new(format!(
                                                "Buy Kernel price label - {quantity}"
                                            )),
                                            TextBundle::from_section(
                                                format!("${:.2}", price),
                                                TextStyle {
                                                    font: font_assets.default.clone(),
                                                    font_size: 20.,
                                                    color: hex(COLOR_BUTTON_TEXT),
                                                },
                                            ),
                                        ));
                                    });
                            }
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

// TODO enable/disable purchase buttons based on account balance and cost
// TODO display quantity and cost for each button

fn button_release_listener(
    mut ev_button_released: EventReader<ButtonReleaseEvent>,
    mut ev_buy_kernel: EventWriter<KernelPurchaseEvent>,
    mut ev_spawn_kernel: EventWriter<KernelSpawnEvent>,
) {
    for ev in ev_button_released.read() {
        match ev.button_type {
            ButtonType::BuyKernel(quantity) => {
                ev_buy_kernel.send(KernelPurchaseEvent { quantity });
                ev_spawn_kernel.send(KernelSpawnEvent { quantity });
                info!("Buy Kernel pressed");
            }
            ButtonType::SellPopcorn(quantity) => {
                // TODO sell popcorn
            }
            ButtonType::Unknown => {
                warn!("Unknown button pressed");
            }
        }
    }
}

fn update_account_balance(
    bank_account: Res<BankAccount>,
    mut q_balance_label: Query<&mut Text, With<AccountBalanceLabel>>,
) {
    let mut text = q_balance_label.single_mut();
    text.sections[0].value = bank_account.to_string();
}

fn update_pop_count(
    pop_counter: Res<PopCounter>,
    mut q_pop_count_label: Query<&mut Text, With<PopCountLabel>>,
) {
    let mut text = q_pop_count_label.single_mut();
    text.sections[1].value = pop_counter.to_string();
}
