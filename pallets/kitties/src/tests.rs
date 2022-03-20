use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_working_create_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		let own = KittiesModule::kitties_owned(1);
		assert_eq!(own.len(), 1);

		let id = own[0];
		let kitty = KittiesModule::kitties(&id).unwrap();
		assert_eq!(kitty.owner, 1);
	});
}

#[test]
fn should_working_set_price() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		let kitty_id = KittiesModule::mint(&1, None, None).unwrap();
		assert_ok!(KittiesModule::set_price(Origin::signed(1), kitty_id, Some(100)));

		// Read pallet storage and assert an expected result.

		let kitty = KittiesModule::kitties(&kitty_id).unwrap();
		assert_eq!(kitty.price, Some(100));
	});
}

#[test]
fn should_not_working_set_price() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		let kitty_id = KittiesModule::mint(&1, None, None).unwrap();
		assert_noop!(KittiesModule::set_price(Origin::signed(2), kitty_id, Some(100)), Error::<Test>::NotKittyOwner);
	});
}

#[test]
fn should_working_transfer() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		let kitty_id = KittiesModule::mint(&1, None, None).unwrap();
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, kitty_id));

		let kitty = KittiesModule::kitties(&kitty_id).unwrap();
		assert_eq!(kitty.owner, 2);
	});
}

#[test]
fn should_working_buy() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		let kitty_id = KittiesModule::mint(&1, None, None).unwrap();
		assert_ok!(KittiesModule::set_price(Origin::signed(1), kitty_id, Some(100)));

		let before_balance = Balances::free_balance(&2);
		assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), kitty_id, 101));
		
		// check owner
		let kitty = KittiesModule::kitties(&kitty_id).unwrap();
		assert_eq!(kitty.owner, 2);
		
		// check buyer balance
		let after_balance = Balances::free_balance(&2);
		assert_eq!(before_balance - 101, after_balance);
	});
}
