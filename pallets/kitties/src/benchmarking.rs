//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
// use frame_support::traits::{ConstU16, ConstU64, ConstU32, ConstU128};
use frame_system::RawOrigin;
use frame_support::{
  sp_runtime::traits::Hash,
  traits::{ Currency },
};

benchmarks! {
	// create_kitty {
	// 	let s in 0 .. 100;
	// 	let caller: T::AccountId = whitelisted_caller();
	// }: _(RawOrigin::Signed(caller))

	buy_kitty {
		let s in 0 .. 100;
		let buyer: T::AccountId = whitelisted_caller();
		let seller: T::AccountId = account("seller", 0, 0);
        // mint
		let id = Kitties::<T>::mint(&seller, None, None).unwrap();
        // let price = T::Currency::minimum_balance();
        let price = 100u32.into();
        // set price
		Kitties::<T>::set_price(RawOrigin::Signed(seller).into(), id, Some(price));
        // set buyer balance
        T::Currency::make_free_balance_be(&buyer, 150u32.into());
	}: _(RawOrigin::Signed(buyer), id, price)

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
// ./target/release/node-template benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_kitties --extrinsic '*' --steps 20 --repeat 10 --json-file=raw.json --output ./pallets/kitties/src/weights.rs