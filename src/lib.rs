#![feature(map_first_last)]
#![feature(entry_insert)]
#![feature(specialization)]
#![feature(associated_type_defaults)]
#![feature(once_cell)]
#![feature(trivial_bounds)]

/// Integration with ABCI (gated by `abci` feature).
#[cfg(feature = "abci")]
pub mod abci;

pub mod call;

pub mod client;

/// Data structures which implement the [`state::State`](state/trait.State.html)
/// trait.
pub mod collections;

/// Traits for deterministic encoding and decoding.
///
/// This module is actually just a re-export of the [ed](https://docs.rs/ed)
/// crate.
pub mod encoding;

/// Integration with [merk](https://docs.rs/merk) (gated by `merk` feature).
#[cfg(feature = "merk")]
pub mod merk;

pub mod query;

/// High-level abstractions for state data.
pub mod state;

/// Helpers for executing state machine logic.
pub mod state_machine;

/// Low-level key/value store abstraction.
pub mod store;

/// Tendermint process handler.
pub mod tendermint;

pub mod contexts;

mod error;

// re-exports
pub use error::*;
pub use orga_macros as macros;

pub mod prelude {
    #[cfg(feature = "abci")]
    pub use crate::abci::*;
    pub use crate::collections::*;
    pub use crate::state::*;
    pub use crate::store::*;
    pub use crate::Result;
}
