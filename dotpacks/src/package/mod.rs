pub mod name;
pub mod version;

use std::error::Error;

use async_trait::async_trait;

use self::{name::DotPackageName, version::DotPackageVersion};

#[async_trait]
pub trait DotPackage<Context> {
  type Version: DotPackageVersion;

  /// The identifier of the package.
  fn name(&self) -> DotPackageName;

  // Return Some([`Self::Version`]) if the package is installed,
  // otherwise return None.
  //
  //[`Self::Version`]: DotPackageVariant::Version
  async fn check(&self, context: &Context) -> Option<Self::Version>;

  // Return the version of package which should be installed.
  async fn select_version(&self, context: &Context) -> Self::Version;

  // Install the package with the version.
  async fn install(&self, context: &Context, version: Self::Version) -> Result<(), Box<dyn Error>>;
}
