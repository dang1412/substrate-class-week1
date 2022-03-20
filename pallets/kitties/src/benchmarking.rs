//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	buy_kitty {
		let s in 0 .. 100;
		let buyer: T::AccountId = whitelisted_caller();
		let seller: T::AccountId = account("seller", 0, 0);
		let id = Kitties::mint(&seller, None, None).unwrap();
		// TODO
		Kitties::set_price(RawOrigin::Signed(seller), id, Some(100));
	}: _(RawOrigin::Signed(buyer), id, 100)

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
// ./target/release/node-template benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_kitties --extrinsic '*' --steps 20 --repeat 10 --json-file=raw.json --output ./pallets/kitties/src/weights.rs