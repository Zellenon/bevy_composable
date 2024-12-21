// Demonstrating 3 ways to do the same thing: Spawn an entity with the Torso component and a
// heirarchy of children for each bodypart.

use bevy::{
    app::{App, AppExit, Startup},
    color::Color,
    core::Name,
    prelude::{ClearColor, Commands, Component, PluginGroup},
    DefaultPlugins,
};
use bevy_composable::{app_impl::ComponentTreeable, wrappers::name};
use bevy_composable::{
    app_impl::{ComplexSpawnable, FuncTreeable},
    tree::ComponentTree,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component, Default, Clone)]
pub struct BodyPart;

#[derive(Component, Default, Clone)]
pub struct Torso;

#[derive(Component, Default, Clone)]
pub struct Arm;

#[derive(Component, Default, Clone)]
pub struct Leg;

#[derive(Component, Default, Clone)]
pub struct Hand;

#[derive(Component, Default, Clone)]
pub struct Foot;

pub fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.build());

    app.insert_resource(ClearColor(Color::srgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )));

    app.add_systems(Startup, compose_demonstration);
    app.add_plugins(WorldInspectorPlugin::new());

    app.run()
    //Ok(())
}

fn compose_demonstration(mut commands: Commands) {
    // Method 1
    commands.compose(all_at_once());
    // Method 2
    commands.compose(recursive());
    // Method 3
    commands.compose(variables());
}

// Method 1 - The notation for declaring children causes linters to automatically format declared
// trees in a shape that represents their actual layout.
fn all_at_once() -> ComponentTree {
    (Torso.store() + BodyPart.store() + name("Torso 1"))
        << ((Arm.store() + BodyPart.store() + name("Arm"))
            << (Hand.store() + BodyPart.store() + name("Hand")))
        << ((Arm.store() + BodyPart.store() + name("Arm"))
            << (Hand.store() + BodyPart.store() + name("Hand")))
        << ((Leg.store() + BodyPart.store() + name("Leg"))
            << (Foot.store() + BodyPart.store() + name("Foot")))
        << ((Leg.store() + BodyPart.store() + name("Leg"))
            << (Foot.store() + BodyPart.store() + name("Foot")))
}

// Method 2 - For repeated parts, making functions that use other functions can make assembling
// complex entities easy. I often use this approach for bullets and explosions.
fn recursive() -> ComponentTree {
    Torso.store() + BodyPart.store() + name("Torso 2") << arm() << arm() << leg() << leg()
}

fn hand() -> ComponentTree {
    Hand.store() + BodyPart.store() + name("Hand")
}

fn arm() -> ComponentTree {
    Arm.store() + BodyPart.store() + name("Arm") << hand()
}

fn foot() -> ComponentTree {
    Foot.store() + BodyPart.store() + name("Foot")
}

fn leg() -> ComponentTree {
    Leg.store() + BodyPart.store() + name("Leg") << foot()
}

// Method 3 - note how you can use the same variable multiple times. This means you can store
// component trees if you need/want to.
fn variables() -> ComponentTree {
    let bp = BodyPart.store();
    let arm = bp.clone() + name("Arm") + Arm.store() << (bp.clone() + name("Hand") + Hand.store());
    let leg = bp.clone() + Leg.store() + name("Leg") << (bp.clone() + name("Foot") + Foot.store());
    Torso.store() + name("Torso 3") + bp << arm.clone() << arm << leg.clone() << leg
}
