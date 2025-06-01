use bevy::ecs::name::Name;

use crate::app_impl::ComponentTreeable;

use crate::tree::ComponentTree;

pub fn name<T: Into<String>>(name: T) -> ComponentTree {
    Name::new(name.into().clone()).store()
}
