# Bevy-Composable

[![crates.io](https://img.shields.io/crates/v/bevy_composable.svg)](https://crates.io/crates/bevy_composable)
[![docs](https://docs.rs/bevy_composable/badge.svg)](https://docs.rs/bevy_composable)
[![license](https://img.shields.io/crates/l/bevy_composable)](https://github.com/zellenon/bevy_composable#license)
[![crates.io](https://img.shields.io/crates/d/bevy_composable.svg)](https://crates.io/crates/bevy_composable)

This [Bevy][bevy] library trivializes the process of making, storing, and
combining 'prefabs' for game entities that are made of often-reused groups of
components configured in a given fashion. The `.store()` function, implemented
for all Bundles, allows you to turn a Bundle into a "component tree" which can
be stored (thread-safe!) and cloned, able to be spawned later via the `compose`
command.

No Plugin is necessary! You'll just need to import the trait which adds
`compose` to `commands` (the ComplexSpawnable trait in app_impl).

The `main` branch and the latest release support Bevy version `0.16`. The oldest
bevy version supported is `0.11`

```rust no_run
use bevy::prelude::*;
use bevy_composable::*;

fn startup_system(mut commands: Commands) {
    commands.compose(com_tree_function(some_args) + other_function());
}

fn com_tree_function(some_args: usize) -> ComponentTree {
    (Bundle1, Bundle2(some_args + 5)).store()
}

fn com_tree_function() -> ComponentTree {
    (Bundle1, Bundle2, Bundle3::default(), Bundle4(x,y,z)).store()
}
```

The [example](example) example showcases all the different kinds of fields that
an asset collection can contain using only derive macro attributes.

## Compatible Bevy versions

| `bevy_composable` | `bevy` |
| :---------------- | :----- |
| `0.1` -           | `0.11` |
| `0.2` -           | `0.13` |
| `0.3` -           | `0.14` |
| `0.4` -           | `0.15` |
| branch `main`     | `0.16` |

## License

- MIT license ([LICENSE-MIT](/LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MIT license, shall be dual
licensed as above, without any additional terms or conditions.

[bevy]: https://bevyengine.org/
