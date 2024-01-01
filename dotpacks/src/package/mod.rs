pub mod name;

use futures::future::BoxFuture;

use self::name::DotPackageName;

///
///
/// # Example
/// ```no_run
/// use dotpacks::package::DotPackage;
/// use futures::FutureExt;
/// use dotpacks::name;
///
/// type Ctx = ();
/// fn example_package() -> DotPackage<Ctx> {
///   DotPackage::<Ctx> {
///     name: name!(example),
///     install: |_: &()| async move {
///       // your fancy install actions
///     }.boxed()
///   }
/// }
///
/// ```
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DotPackage<Context> {
  /// The name of the package.
  /// This name is used to display the informations to console.
  pub name: DotPackageName,
  pub install: fn(&Context) -> BoxFuture<'static, ()>,
}
