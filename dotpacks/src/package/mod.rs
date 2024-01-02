pub mod name;
pub mod version;

use std::error::Error;
use std::fmt::Debug;

use async_trait::async_trait;

use self::{name::DotPackageName, version::DotPackageVersion};

pub(crate) type GenericDotPackage<Context> =
  Box<dyn DotPackage<Context, Version = dyn DotPackageVersion>>;

#[async_trait]
pub trait DotPackage<Context>: Debug {
  type Version: DotPackageVersion;

  /// The identifier of the package.
  fn name(&self) -> DotPackageName;

  /// The packages depends on this package.
  fn deps(&self) -> Vec<GenericDotPackage<Context>>;

  // Return Some([`Self::Version`]) if the package is installed,
  // otherwise return None.
  //
  //[`Self::Version`]: DotPackage::Version
  async fn check(&self, context: &Context) -> Option<Self::Version>;

  // Return the version of package which should be installed.
  async fn select_version(&self, context: &Context) -> Self::Version;

  // Install the package with the version.
  async fn install(&self, context: &Context, version: Self::Version) -> Result<(), Box<dyn Error>>;
}
