use crate::app_impl::ComponentTreeable;
use bevy::core::Name;

use crate::tree::ComponentTree;

pub fn name<T: Into<String>>(name: T) -> ComponentTree {
    Name::new(name.into().clone()).store()
}
