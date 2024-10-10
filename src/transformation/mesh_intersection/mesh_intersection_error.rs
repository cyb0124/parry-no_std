use crate::shape::TriMeshBuilderError;

/// Error indicating that a query is not supported between certain shapes
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MeshIntersectionError {
    /// At least one of the meshes is missing its topology information. Call `mesh.compute_topology` on the mesh
    MissingTopology,
    /// At least one of the meshes is missing its pseudo-normals. Call `mesh.compute_pseudo_normals` on the mesh
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
