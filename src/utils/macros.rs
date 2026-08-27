//! General-purpose macros used across `PumpkinPlus`.

/// Produces a `const` namespaced identifier from the crate name and a suffix.
///
/// # Example
///
/// ```rust,ignore
/// const EMBERTREAD_ID: &str = namespaced_id!("embertread");
/// ```
#[macro_export]
macro_rules! namespaced_id {
    ($suffix:expr) => {
        concat!(env!("CARGO_PKG_NAME"), ":", $suffix)
    };
}
