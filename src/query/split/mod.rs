pub use self::split::{IntersectResult, SplitResult};

mod split;
mod split_aabb;
mod split_segment;

#[cfg(feature = "dim3")]
mod split_trimesh;
