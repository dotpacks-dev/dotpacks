use crate::package::{version::DotPackageVersion, DotPackage};

// #[derive(Debug, Clone)]
pub struct DotpacksRunner<Context> {
  config: Dotpacks<Context>,
}

/// Holding all information about the dotpacks configuration, that is,
/// anything you want to manage with dotpacks.
///
/// `Context` is the type of the struct that is passed to all configuration.
/// If you want change some configuration by such environment,
/// you can make your config with `Context`, and run with it.
// #[derive(Debug, Clone)]
pub struct Dotpacks<Context> {
  packages: Vec<Box<dyn DotPackage<Context, Version = dyn DotPackageVersion>>>,
}

impl<Context> Dotpacks<Context> {
  pub fn new() -> Self {
    Dotpacks::<Context> {
      packages: Vec::<Box<dyn DotPackage<Context, Version = dyn DotPackageVersion>>>::new(),
    }
  }
}

impl<Context> Default for Dotpacks<Context> {
  fn default() -> Self {
    Self::new()
  }
}

impl<Context> From<Dotpacks<Context>> for DotpacksRunner<Context> {
  fn from(val: Dotpacks<Context>) -> Self {
    DotpacksRunner::<Context> { config: val }
  }
}

impl<Context> DotpacksRunner<Context> {
  pub fn run(&self, _context: Context) {
    println!("{} packages found.", self.config.packages.len());
    todo!();
  }
}
