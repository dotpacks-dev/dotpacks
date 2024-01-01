use futures::future::BoxFuture;

///
///
/// # Example
/// ```no_run
/// use dotpacks::package::DotPackage;
/// use futures::FutureExt;
///
/// let _ = DotPackage::<()> {
///   name: "example".into(),
///   install: |_: &()| async move {
///     println!("Hello, world!")
///   }.boxed(),
/// };
/// ```
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct DotPackage<Context> {
  /// The name of the package.
  /// This name is used to display the informations to console.
  pub name: String,
  pub install: fn(&Context) -> BoxFuture<'static, ()>,
}
