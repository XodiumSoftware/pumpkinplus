/// A trait representing a generic module with an `enabled` status.
///
/// This trait is intended to be implemented by types that represent
/// a module or component that can be enabled or disabled. The implementation
/// of the `enabled` method provides a way to query the current status of the module.
pub trait Module {
    /// Returns `true` if the module is enabled, `false` otherwise.
    fn enabled(&self) -> bool;
}
