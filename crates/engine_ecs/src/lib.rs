/**
 * path: /crates/engine_ecs/src/lib.rs
 * description: Entity-Component-System (ECS) module
 */

pub use bevy_ecs as ecs;
// pub use bevy_reflect as reflect;
// Reflection will be added later when the inspector needs it.
// For now we keep ECS minimal to avoid feature/version friction.