pub use once_cell::sync::Lazy;
use std::{
  fmt::{Display, Formatter},
  sync::atomic::AtomicU64,
};

#[derive(Debug, Clone)]
pub struct DotPackageName {
  pub display: String,
  pub unique: u64,
}

impl Display for DotPackageName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.display)
  }
}

impl PartialEq for DotPackageName {
  fn eq(&self, other: &Self) -> bool {
    self.unique == other.unique
  }
}

impl Eq for DotPackageName {}

pub static DOTPACKAGES_UNIQUE: AtomicU64 = AtomicU64::new(0);

/// This macro is used to create a `DotPackagesName` constant.
///
/// This macro enable us to re-use package.
/// Although you generate packages from functions, dotpacks can know they'are same.
/// ```
/// # use pretty_assertions::{assert_eq, assert_ne};
/// use dotpacks::name;
/// use dotpacks::package::DotPackage;
/// use futures::FutureExt;
/// type Ctx = ();
///
/// fn example_package() -> DotPackage<Ctx> {
///   DotPackage::<Ctx> {
///     name: name!(example),
///     install: |_: &()| async move {}.boxed()
///   }
/// }
///
/// let instance_1 = example_package();
/// let instance_2 = example_package();
///
/// assert_eq!(instance_1.name, instance_2.name);
///
/// ```
///
///  This macro produces a truly unique value each time it is called,
///  so there is no need to worry about name conflicts.
///  ```
/// # use pretty_assertions::{assert_eq, assert_ne};
/// use dotpacks::name;
/// use dotpacks::package::DotPackage;
/// use futures::FutureExt;
/// type Ctx = ();
///
/// fn package_a() -> DotPackage<Ctx> {
///   DotPackage::<Ctx> {
///     name: name!(example),
///     install: |_: &()| async move {}.boxed()
///   }
/// }
///
/// fn package_b() -> DotPackage<Ctx> {
///   DotPackage::<Ctx> {
///     name: name!(example),
///     install: |_: &()| async move {}.boxed()
///   }
/// }
///
/// let a = package_a();
/// let b = package_b();
///
/// // These packages have same display name
/// println!("a: {}", a.name); // -> a: example
/// println!("b: {}", b.name); // -> b: example
///
/// // But these packages have different unique value,
/// // so dotpacks can know they are different.
/// assert_ne!(a.name, b.name);
///
/// ```
///
///
/// # Example
/// ```
/// # use pretty_assertions::assert_eq;
/// use dotpacks::name;
/// use dotpacks::package::DotPackage;
/// use futures::FutureExt;
/// type Ctx = ();
///
/// fn example_package() -> DotPackage<Ctx> {
///   DotPackage::<Ctx> {
///     name: name!(example),
///     install: |_: &()| async move {}.boxed()
///   }
/// }
///
/// # assert_eq!(example_package().name.display, "example");
/// # assert_eq!(example_package().name.unique, example_package().name.unique);
///
/// ```
///
#[macro_export]
macro_rules! name {
  ($name:ident) => {
    $crate::package::name::DotPackageName {
      display: stringify!($name).into(),
      unique: {
        static UNIQUE: $crate::package::name::Lazy<u64> = $crate::package::name::Lazy::new(|| {
          $crate::package::name::DOTPACKAGES_UNIQUE
            .fetch_add(1, ::std::sync::atomic::Ordering::Relaxed)
        });
        *UNIQUE
      },
    }
  };
}

#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_ne};

  use super::*;

  #[test]
  fn dotpackagename_should_be_unique() {
    assert_eq!(package_a_1().display, "package_a");
    assert_eq!(package_a_2().display, "package_a");
    assert_eq!(package_b().display, "package_b");
    assert_eq!(format!("{}", package_a_1()), "package_a");
    assert_eq!(format!("{}", package_a_2()), "package_a");
    assert_eq!(format!("{}", package_b()), "package_b");

    assert_eq!(package_a_1().unique, package_a_1().unique);
    assert_eq!(package_a_2().unique, package_a_2().unique);
    assert_eq!(package_b().unique, package_b().unique);
    assert_ne!(package_a_1().unique, package_a_2().unique);
    assert_ne!(package_a_1().unique, package_b().unique);
    assert_ne!(package_a_2().unique, package_b().unique);
  }

  fn package_a_1() -> DotPackageName {
    name!(package_a)
  }

  fn package_a_2() -> DotPackageName {
    name!(package_a)
  }

  fn package_b() -> DotPackageName {
    name!(package_b)
  }
}
