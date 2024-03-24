use clap::ValueEnum;

/// Specifies resources to sync on monitor
pub struct ResourceFile {
  pub builders: (),
  pub servers: (),
  pub builds: (),
  pub deployments: (),
  pub repos: (),
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SyncDirection {
  /// Brings up resources / updates
  Up,
  /// Takes down / deletes resources
  Down,
}
