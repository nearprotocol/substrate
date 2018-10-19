// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate Consensus Common.

// Substrate Demo is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate Consensus Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate Consensus Common.  If not, see <http://www.gnu.org/licenses/>.

//! Tracks offline validators.
#![recursion_limit="128"]

extern crate substrate_primitives as primitives;
extern crate futures;
extern crate sr_version as runtime_version;
extern crate sr_primitives as runtime_primitives;
extern crate tokio;

#[macro_use]
extern crate error_chain;

use std::sync::Arc;

use primitives::{ed25519, AuthorityId};
use runtime_primitives::{generic::BlockId, traits::Block, Justification};
use futures::prelude::*;

pub mod offline_tracker;
pub mod error;

pub use self::error::{Error, ErrorKind};

/// Block import trait.
pub trait BlockImport<B: Block> {
	/// Import a block alongside its corresponding justification.
	fn import_block(&self, block: B, justification: Justification, authorities: &[AuthorityId]) -> bool;
}

/// Trait for getting the authorities at a given block.
pub trait Authorities<B: Block> {
	/// Get the authorities at the given block.
	fn authorities(&self, at: &BlockId<B>) -> Result<Vec<AuthorityId>, Error>;
}

/// Environment producer for a BFT instance. Creates proposer instance and communication streams.
pub trait Environment<B: Block> {
	/// The proposer type this creates.
	type Proposer: Proposer<B>;
	/// Error which can occur upon creation.
	type Error: From<Error>;

	/// Initialize the proposal logic on top of a specific header.
	fn init(&self, parent_header: &B::Header, authorities: &[AuthorityId], sign_with: Arc<ed25519::Pair>)
		-> Result<Self::Proposer, Self::Error>;
}

/// Logic for a proposer.
///
/// This will encapsulate creation and evaluation of proposals at a specific
/// block.
pub trait Proposer<B: Block> {
	/// Error type which can occur when proposing or evaluating.
	type Error: From<Error> + ::std::fmt::Debug + 'static;
	/// Future that resolves to a committed proposal.
	type Create: IntoFuture<Item=B,Error=Self::Error>;
	/// Future that resolves when a proposal is evaluated.
	type Evaluate: IntoFuture<Item=bool,Error=Self::Error>;

	/// Create a proposal.
	fn propose(&self) -> Self::Create;

	/// Evaluate proposal. True means valid.
	fn evaluate(&self, proposal: &B) -> Self::Evaluate;
}
