use bevy::{ecs::system::EntityCommands, prelude::Entity};

use crate::{app_impl::spawn_complex_inner, tree::ComponentTree};

pub trait ComplexSpawnable {
    fn spawn_complex(&mut self, tree: ComponentTree) -> Entity;
}

impl ComplexSpawnable for EntityCommands<'_> {
    fn spawn_complex(&mut self, tree: ComponentTree) -> Entity {
        spawn_complex_inner(self, &tree);
        self.id()
    }
}
