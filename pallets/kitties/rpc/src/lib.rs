// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the transaction payment pallet.

pub use self::gen_client::Client as TransactionPaymentClient;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT},
};
use std::sync::Arc;

use pallet_kitties_rpc_runtime_api::KittyApi as KittyRuntimeApi;
use pallet_kitties::{ Kitty, Config };
use codec::Codec;
use std::fmt::Display;

#[rpc]
pub trait KittyApi<BlockHash, T> where T: Config + Codec, {
	#[rpc(name = "kitty_cnt")]
    fn get_kitty_cnt(&self, at: Option<BlockHash>) -> Result<u64>;
    fn get_kitty(&self, id: T::Hash, at: Option<BlockHash>) -> Result<Kitty<T>>;
}

/// A struct that implements the [`TransactionPaymentApi`].
pub struct KittyStorage<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> KittyStorage<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, T> KittyApi<<Block as BlockT>::Hash, T>
	for KittyStorage<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: KittyRuntimeApi<Block, T>,
    T: Config + Codec,
    // Kitty<T>: sp_api::Decode,
{
	fn get_kitty_cnt(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let result_api = api.get_kitty_cnt(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

    fn get_kitty(
		&self,
        id: T::Hash,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Kitty<T>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let result_api = api.get_kitty(&at, id);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}
}