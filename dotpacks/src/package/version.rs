use dyn_ord::DynEq;
use std::fmt::Debug;

/// The trait which the version returned by [`DotPackage`] should satisfy.
///
/// [`DotPackage`]: crate::package::DotPackage
///
/// This trait has same meaning as the following:
/// ```ignore
/// trait DotPackageVersion = Debug + PartialEq + Serialize + Send + Sync;
/// ```
/// But this feature(traits alias) is still unstable, so we implemented it manually.
///
pub trait DotPackageVersion: Debug + DynEq + erased_serde::Serialize + Send + Sync {}
impl<T: Debug + DynEq + erased_serde::Serialize + Send + Sync> DotPackageVersion for T {}
