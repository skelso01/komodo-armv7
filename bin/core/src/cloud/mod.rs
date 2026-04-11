#[derive(Debug)]
pub enum BuildCleanupData {
  /// Nothing to clean up
  Server,
  /// Cleanup Periphery connection
  Url,
}
