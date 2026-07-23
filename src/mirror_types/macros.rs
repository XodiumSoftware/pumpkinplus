/// Generates a mirror enum with serde, `Display`, `matches_config`, and `From` impls.
///
/// # Usage
/// ```
/// mirror_enum! {
///     pub enum GameMode from pumpkin_plugin_api::player::GameMode {
///         Survival,
///         Creative,
///         Adventure,
///         Spectator,
///     }
/// }
/// ```
///
/// This produces:
/// - An enum with `Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash`
/// - `#[serde(rename_all = "PascalCase")]`
/// - `matches_config(&self, allowed: &[Self]) -> bool`
/// - `Display` via `Debug` representation
/// - `From<upstream>` with a catch-all fallback to the first variant
#[macro_export]
macro_rules! mirror_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident from $upstream:path {
            $first:ident,
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
        #[serde(rename_all = "PascalCase")]
        $vis enum $name {
            $first,
            $($variant),*
        }

        impl $name {
            /// Returns true if the given list is empty (allow-all) or contains this variant.
            pub fn matches_config(&self, allowed: &[Self]) -> bool {
                allowed.is_empty() || allowed.contains(self)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl From<$upstream> for $name {
            fn from(value: $upstream) -> Self {
                #[allow(unreachable_patterns)]
                match value {
                    <$upstream>::$first => Self::$first,
                    $(<$upstream>::$variant => Self::$variant,)*
                    _ => Self::$first,
                }
            }
        }
    };
}
