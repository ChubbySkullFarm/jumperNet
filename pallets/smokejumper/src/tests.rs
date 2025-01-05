use crate::{mock::*, Error, Event, Smokejumpers};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_smj() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Name
		let smj_name: Vec<u8> = "Skeebo".as_bytes().to_vec().try_into().unwrap();
		let bounded_name: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from(smj_name).unwrap();

		assert_eq!(Smokejumpers::<Test>::contains_key(1), false);

		// Create SMJ
		assert_ok!(Smokejumper::create_smj(RuntimeOrigin::signed(1), 1, bounded_name.clone()));

		// Check
		assert_eq!(Smokejumpers::<Test>::contains_key(1), true);
		assert_noop!(
			Smokejumper::create_smj(RuntimeOrigin::signed(1), 1, bounded_name.clone()),
			Error::<Test>::SmjAlreadyCreated,
		);

		assert_eq!(Balances::free_balance(1), 420_000_000_000_000);
		System::assert_last_event(Event::SmjCreated { smj: bounded_name }.into());
	});
}

#[test]
fn change_name() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Old Name
		let smj_name: Vec<u8> = "Skeebo".as_bytes().to_vec().try_into().unwrap();
		let bounded_name: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from(smj_name).unwrap();

		// New Name
		let new_name: Vec<u8> = "Big Ern".as_bytes().to_vec().try_into().unwrap();
		let new_bounded: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from(new_name).unwrap();

		//Create SMJ
		assert_ok!(Smokejumper::create_smj(RuntimeOrigin::signed(1), 1, bounded_name.clone()));

		let test_data = Smokejumpers::<Test>::get(1).unwrap();
		assert_eq!(test_data.name, bounded_name.clone());

		// Change name
		assert_ok!(Smokejumper::change_name(RuntimeOrigin::signed(1), 1, new_bounded.clone()));

		// Check
		let new_data = Smokejumpers::<Test>::get(1).unwrap();
		assert_eq!(new_data.name, new_bounded.clone());

		assert_noop!(
			Smokejumper::change_name(RuntimeOrigin::signed(1), 2, new_bounded.clone()),
			Error::<Test>::SmjDoesNotExist,
		);
	});
}

#[test]
fn swap_address() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Name
		let smj_name: Vec<u8> = "Skeebo".as_bytes().to_vec().try_into().unwrap();
		let bounded_name: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
			BoundedVec::try_from(smj_name).unwrap();

		// Create SMJ & Swap data to new address
		assert_ok!(Smokejumper::create_smj(RuntimeOrigin::signed(1), 1, bounded_name.clone()));
		assert_ok!(Smokejumper::swap_address(RuntimeOrigin::signed(1), 1, 2));

		// Check balances
		assert_eq!(Balances::free_balance(2), 420_000_000_000_000);
		assert_eq!(Balances::free_balance(1), 1);

		// Check data
		let new_smj = Smokejumpers::<Test>::get(2).unwrap();
		assert_eq!(new_smj.name, bounded_name.clone());
	});
}
