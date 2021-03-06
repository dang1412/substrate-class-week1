
//! Autogenerated weights for `pallet_template`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-18, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// C:\Users\dttun\OneDrive\Documents\Projects\near\substrate-class-week1\target\release\node-template.exe
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_template
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --json-file=raw.json
// --output
// ./pallets/template/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn do_something() -> Weight;
}

/// Weight functions for `pallet_template`.
pub struct SubstrateWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeightInfo<T> {
	// Storage: TemplateModule Something (r:0 w:1)
	fn do_something() -> Weight {
		(30_807_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

impl WeightInfo for () {
	fn do_something() -> Weight {
		(30_807_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
