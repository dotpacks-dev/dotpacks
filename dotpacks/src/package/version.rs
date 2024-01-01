use dyn_ord::DynEq;

/// The trait which the version returned by [`DotPackage`] should satisfy.
///
/// [`DotPackage`]: crate::package::DotPackage
///
/// This trait has same meaning as the following:
/// ```ignore
/// trait DotPackageVersion = PartialEq + Eq + Serialize + DeserializeOwned + Send + Sync;
/// ```
/// But this feature(traits alias) is still unstable, so we implemented it manually.
///
pub trait DotPackageVersion: DynEq + erased_serde::Serialize + Send + Sync {}
impl<T: DynEq + erased_serde::Serialize + Send + Sync> DotPackageVersion for T {}
