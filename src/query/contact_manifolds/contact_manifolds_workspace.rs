#![allow(clippy::multiple_bound_locations)] // for impl_downcast

use alloc::boxed::Box;
use downcast_rs::{impl_downcast, DowncastSync};

use crate::query::contact_manifolds::{
    CompositeShapeCompositeShapeContactManifoldsWorkspace,
    CompositeShapeShapeContactManifoldsWorkspace,
    HeightFieldCompositeShapeContactManifoldsWorkspace, HeightFieldShapeContactManifoldsWorkspace,
    TriMeshShapeContactManifoldsWorkspace,
};

#[derive(Copy, Clone)]
/// Enum representing workspace data of a specific type.
pub enum TypedWorkspaceData<'a> {
    /// A trimesh workspace.
    TriMeshShapeContactManifoldsWorkspace(&'a TriMeshShapeContactManifoldsWorkspace),
    /// A heightfield vs. shape workspace.
    HeightfieldShapeContactManifoldsWorkspace(&'a HeightFieldShapeContactManifoldsWorkspace),
    /// A heightfield vs. composite shape workspace.
    HeightfieldCompositeShapeContactManifoldsWorkspace(
        &'a HeightFieldCompositeShapeContactManifoldsWorkspace,
    ),
    /// A composite shape vs. composite shape workspace.
    CompositeShapeCompositeShapeContactManifoldsWorkspace(
        &'a CompositeShapeCompositeShapeContactManifoldsWorkspace,
    ),
    /// A composite shape vs. shape workspace.
    CompositeShapeShapeContactManifoldsWorkspace(&'a CompositeShapeShapeContactManifoldsWorkspace),
    /// A custom workspace.
    Custom,
}

/// Data from a [`ContactManifoldsWorkspace`].
pub trait WorkspaceData: DowncastSync {
    /// Gets the underlying workspace as an enum.
    fn as_typed_workspace_data(&self) -> TypedWorkspaceData;

    /// Clones `self`.
    fn clone_dyn(&self) -> Box<dyn WorkspaceData>;
}

impl_downcast!(sync WorkspaceData);

// Note we have this newtype because it simplifies the serialization/deserialization code.
/// A serializable workspace used by some contact-manifolds computation algorithms.
pub struct ContactManifoldsWorkspace(pub Box<dyn WorkspaceData>);

impl Clone for ContactManifoldsWorkspace {
    fn clone(&self) -> Self {
        ContactManifoldsWorkspace(self.0.clone_dyn())
    }
}

impl<T: WorkspaceData> From<T> for ContactManifoldsWorkspace {
    fn from(data: T) -> Self {
        Self(Box::new(data) as Box<dyn WorkspaceData>)
    }
}
