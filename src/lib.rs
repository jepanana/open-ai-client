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

mod audio;
mod beta;
mod chat_completion;
mod client;
mod client_builder;
mod common;
mod embeddings;
mod files;
mod fine_tunning;
mod images;
mod models;
mod moderations;

pub use self::audio::*;
pub use self::beta::*;
pub use self::chat_completion::*;
pub use self::common::*;
pub use self::embeddings::*;
pub use self::files::*;
pub use self::fine_tunning::*;
pub use self::images::*;
pub use self::models::*;
pub use self::moderations::*;

pub use self::client::*;
pub use self::client_builder::*;
pub use self::common::OpenAIError;
