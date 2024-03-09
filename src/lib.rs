//! API workspace
#![deny(
    bad_style,
    clippy::wildcard_imports,
    dead_code,
    deprecated,
    improper_ctypes,
    missing_debug_implementations,
    missing_docs,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_bounds,
    private_interfaces,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unknown_lints,
    unreachable_code,
    unreachable_pub,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_results,
    warnings,
    while_true
)]

#[macro_use]
extern crate tracing;

#[cfg(feature = "stable")]
mod audio;
mod base_client;
mod beta;
mod chat_completion;
mod client;
mod client_builder;
mod common;
mod embeddings;
mod files;
mod fine_tunning;
mod images;

#[cfg(feature = "models")]
pub mod models;

#[cfg(feature = "moderations")]
pub mod moderations;

#[cfg(feature = "stable")]
pub use self::audio::*;

pub use self::beta::*;

#[cfg(feature = "stable")]
pub use self::chat_completion::*;

#[cfg(any(feature = "stable", feature = "beta"))]
pub use self::common::*;

#[cfg(feature = "stable")]
pub use self::embeddings::*;

#[cfg(feature = "stable")]
pub use self::files::*;

#[cfg(feature = "stable")]
pub use self::fine_tunning::*;

#[cfg(feature = "stable")]
pub use self::images::*;

#[cfg(feature = "models")]
pub use self::models::ModelsHandler;

#[cfg(feature = "moderations")]
pub use self::moderations::ModerationsHandler;

#[cfg(feature = "stable")]
pub use self::client::*;

#[cfg(any(feature = "stable", feature = "beta"))]
pub use self::client_builder::*;

#[cfg(feature = "stable")]
pub use self::common::OpenAIError;
