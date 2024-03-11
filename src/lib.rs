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

mod base_client;
mod beta;
mod client;
mod client_builder;
mod common;

pub use self::client::*;
pub use self::client_builder::*;
pub use self::common::*;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "chat")]
pub mod chat;

#[cfg(feature = "embeddings")]
pub mod embeddings;

#[cfg(feature = "files")]
pub mod files;

#[cfg(feature = "fine_tunning")]
pub mod fine_tunning;

#[cfg(feature = "images")]
pub mod images;

#[cfg(feature = "models")]
pub mod models;

#[cfg(feature = "moderations")]
pub mod moderations;

#[cfg(feature = "assistants")]
pub use self::beta::assistants;

pub use self::beta::common as assistants_common;

#[cfg(feature = "messages")]
pub use self::beta::messages;

#[cfg(feature = "runs")]
pub use self::beta::runs;

#[cfg(feature = "threads")]
pub use self::beta::threads;
