#[derive(Debug, PartialEq)]
/// Errors generated by the convex-hull calculation.
pub enum ConvexHullError {
    /// Reached an impossible configuration in the convex-hull calculation,
    /// likely because of a bug.
    InternalError(&'static str),
    /// The convex hull calculation was unable to find a support point.
    /// This generally happens if the input point set contains invalid points (with NaN coordinates)
    /// or if they are almost coplanar.
    MissingSupportPoint,
    /// The convex-hull calculation failed because less than 3 points were provided.
    IncompleteInput,
    /// Reached a piece of code we shouldn’t (internal error).
    Unreachable,
}
