//! Spatial partitioning tools.

pub use self::qbvh::{
    CenterDataSplitter, IndexedData, NodeIndex, Qbvh, QbvhDataGenerator, QbvhNode,
    QbvhNonOverlappingDataSplitter, QbvhProxy, QbvhUpdateWorkspace, SimdNodeIndex,
};
pub use self::visitor::{
    SimdBestFirstVisitStatus, SimdBestFirstVisitor, SimdSimultaneousVisitStatus,
    SimdSimultaneousVisitor, SimdVisitStatus, SimdVisitor, SimdVisitorWithContext,
};

/// A quaternary bounding-volume-hierarchy.
#[deprecated(note = "Renamed to Qbvh")]
pub type SimdQbvh<T> = Qbvh<T>;

mod qbvh;
mod visitor;
