//! Re-export the Dioxus prelude from the active renderer backend.
//!
//! With `desktop` feature: re-exports from `dioxus::prelude`
//! With `native` feature: re-exports from `dioxus_native::prelude`

#[cfg(feature = "desktop")]
pub use dioxus::prelude::*;

#[cfg(feature = "native")]
pub use dioxus_native::prelude::*;
