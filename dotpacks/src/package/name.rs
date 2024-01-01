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
