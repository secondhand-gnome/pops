use bevy::prelude::*;

pub struct KernelPlugin;

impl Plugin for KernelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_kernel)
            .register_type::<Kernel>()
            .register_type::<KernelState>();
    }
}

#[derive(Debug, Default, Reflect)]
enum KernelState {
    #[default]
    Raw,
    Popped,
}

#[derive(Component, Debug, Default, Reflect)]
struct Kernel {
    state: KernelState,
}

#[derive(Bundle, Debug, Default)]
struct KernelBundle {
    kernel: Kernel,
    // TODO sprite and transform
}

fn spawn_first_kernel(mut commands: Commands) {
    commands.spawn((KernelBundle { ..default() }, Name::new("Kernel")));
}
