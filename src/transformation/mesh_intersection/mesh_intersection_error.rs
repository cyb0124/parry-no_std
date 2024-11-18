use crate::shape::TriMeshBuilderError;

#[cfg(doc)]
use crate::shape::{TriMesh, TriMeshFlags};

/// Error indicating that a query is not supported between certain shapes
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MeshIntersectionError {
    /// At least one of the meshes is missing its topology information. Ensure that the [`TriMeshFlags::ORIENTED`] flag is enabled on both meshes.
    MissingTopology,
    /// At least one of the meshes is missing its pseudo-normals. Ensure that the [`TriMeshFlags::ORIENTED`] flag is enabled on both meshes.
    MissingPseudoNormals,
    /// Internal failure while intersecting two triangles
    TriTriError,
    /// Internal failure while merging faces resulting from intersections
    DuplicateVertices,
    /// Internal failure while triangulating an intersection face
    TriangulationError,
    /// See [`TriMeshBuilderError`]
    TriMeshBuilderError(TriMeshBuilderError),
}

impl From<TriMeshBuilderError> for MeshIntersectionError {
    fn from(value: TriMeshBuilderError) -> Self {
        MeshIntersectionError::TriMeshBuilderError(value)
    }
}
