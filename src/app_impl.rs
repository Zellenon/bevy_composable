use std::sync::Arc;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::tree::{ComponentTree, EntityCommandSet};

pub trait ComplexSpawnable {
    fn compose(&mut self, tree: ComponentTree) -> Entity;
}

impl ComplexSpawnable for Commands<'_, '_> {
    fn compose(&mut self, tree: ComponentTree) -> Entity {
        let entity = &mut self.spawn_empty();
        compose_inner(entity, &tree);
        entity.id()
    }
}

fn compose_inner(entity: &mut EntityCommands, component_tree: &ComponentTree) {
    for command in component_tree.commands.iter() {
        command(entity);
    }
    for child in component_tree.children.iter() {
        entity.with_children(|w| {
            let mut child_entity = w.spawn_empty();
            compose_inner(&mut child_entity, child);
        });
    }
}

pub fn from<T>(value: impl Component + Clone) -> EntityCommandSet {
    let func = move |parent: &mut EntityCommands| {
        parent.insert(value.clone());
    };
    Arc::new(func) as EntityCommandSet
}

pub trait ComponentTreeable {
    fn store(self) -> ComponentTree;
}

impl<W> ComponentTreeable for W
where
    W: Bundle + Clone,
{
    fn store(self) -> ComponentTree {
        let func = move |parent: &mut EntityCommands| {
            parent.insert(self.clone());
        };
        (Arc::new(func) as EntityCommandSet).into()
    }
}

pub trait FuncTreeable {
    fn store() -> ComponentTree;
}

impl<W> FuncTreeable for W
where
    W: Bundle + Default,
{
    fn store() -> ComponentTree {
        let func = move |parent: &mut EntityCommands| {
            parent.insert(Self::default());
        };
        (Arc::new(func) as EntityCommandSet).into()
    }
}
