//TODO:
// Repetitious Code
// Import storage differently?
// Better typing so better testing can happen

use crate::{
	mock::*, Drogues, DroguesRiggedNonce, Error, Event, InServiceNonce, Mains, MainsRiggedNonce,
	RepairNonce, Reserves, ReservesRiggedNonce, RetireNonce, Riggers,
};

use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_loft_id() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Create Loft ID
		assert_ok!(Parachutes::create_loft_id(RuntimeOrigin::signed(1), 1));
		assert_noop!(
			Parachutes::create_loft_id(RuntimeOrigin::signed(1), 1),
			Error::<Test>::AlreadyARigger
		);

		// Test Storage
		let test_rigger = Riggers::<Test>::get(1).unwrap().rigger;
		assert_eq!(test_rigger, crate::Rigger::Uncertified);

		// Last Event
		System::assert_last_event(Event::<Test>::NewRigger { who: 1 }.into());
	});
}

#[test]
fn set_rigger() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Senior),
			Error::<Test>::NoLoftId
		);
		assert_ok!(Parachutes::create_loft_id(RuntimeOrigin::signed(1), 1));

		// Set Rigger::Senior
		assert_ok!(Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Senior));
		let test_rigger = Riggers::<Test>::get(1).unwrap().rigger;

		// Test Rigger::Senior
		assert_eq!(test_rigger, crate::Rigger::Senior);

		// Set Rigger::Master
		assert_ok!(Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Master));

		// Test set Rigger::Master to Rigger::Master
		assert_noop!(
			Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Master),
			Error::<Test>::AlreadyARigger
		);

		// Last Event
		System::assert_last_event(
			Event::RiggerChanged { who: 1, rigger: crate::Rigger::Master }.into(),
		);
	});
}

#[test]
fn in_service() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Set SMJ && Loft ID
		// Create Parachute ID && Metadata
		let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
		let parachute_owner: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(owner_name).unwrap();

		let drogue_vec: Vec<u8> = "8DG069".as_bytes().to_vec().try_into().unwrap();
		let drogue_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(drogue_vec).unwrap();

		let main_vec: Vec<u8> = "8DC069".as_bytes().to_vec().try_into().unwrap();
		let main_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(main_vec).unwrap();

		let reserve_vec: Vec<u8> = "8RV069".as_bytes().to_vec().try_into().unwrap();
		let reserve_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(reserve_vec).unwrap();

		let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
		let date: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(date_vec).unwrap();

		assert_ok!(Parachutes::create_loft_id(RuntimeOrigin::signed(1), 1));

		// Get Nonce for In Service Function Calls
		assert_eq!(InServiceNonce::<Test>::get(), 0u32);

		// Test Permissions
		assert_noop!(
			Parachutes::in_service(
				RuntimeOrigin::signed(1),
				1,
				crate::ParachuteType::Drogue,
				parachute_owner.clone(),
				crate::ParachuteModel::Drogue,
				drogue_id.clone(),
				date.clone(),
			),
			Error::<Test>::NotQualified
		);

		// Grant Permissions
		assert_ok!(Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Senior));

		// In-Service Drogue Parachute
		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Drogue,
			parachute_owner.clone(),
			crate::ParachuteModel::Drogue,
			drogue_id.clone(),
			date.clone(),
		));

		// Test In-Service Same Parachute
		assert_noop!(
			Parachutes::in_service(
				RuntimeOrigin::signed(1),
				1,
				crate::ParachuteType::Drogue,
				parachute_owner.clone(),
				crate::ParachuteModel::Drogue,
				drogue_id.clone(),
				date.clone(),
			),
			Error::<Test>::ParachuteAlreadyExists
		);

		// Test In-Service Nonce After Function Call
		assert_eq!(InServiceNonce::<Test>::get(), 1u32);

		// Test Rigger In-Servicing Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let in_service = rigger_details.in_service.get(0).unwrap();
		assert_eq!(*in_service, 0u32);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteInService { who: 1, parachute: drogue_id.clone() }.into(),
		);

		// In-Service Main Parachute
		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Main,
			parachute_owner.clone(),
			crate::ParachuteModel::DC7,
			main_id.clone(),
			date.clone(),
		));

		// Test In-Service Same Parachute
		assert_noop!(
			Parachutes::in_service(
				RuntimeOrigin::signed(1),
				1,
				crate::ParachuteType::Main,
				parachute_owner.clone(),
				crate::ParachuteModel::DC7,
				main_id.clone(),
				date.clone(),
			),
			Error::<Test>::ParachuteAlreadyExists
		);

		// Get In-Service Nonce
		assert_eq!(InServiceNonce::<Test>::get(), 2u32);

		// Test Rigger In-Servicing Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let in_service = rigger_details.in_service.get(1).unwrap();
		assert_eq!(*in_service, 1u32);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteInService { who: 1, parachute: main_id.clone() }.into(),
		);

		// In-Service Reserve Parachute
		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Reserve,
			parachute_owner.clone(),
			crate::ParachuteModel::MT1S,
			reserve_id.clone(),
			date.clone(),
		));

		// Test In-Service Same Parachute
		assert_noop!(
			Parachutes::in_service(
				RuntimeOrigin::signed(1),
				1,
				crate::ParachuteType::Reserve,
				parachute_owner.clone(),
				crate::ParachuteModel::MT1S,
				reserve_id.clone(),
				date.clone(),
			),
			Error::<Test>::ParachuteAlreadyExists
		);

		// Test In-Service Nonce After Function Call
		assert_eq!(InServiceNonce::<Test>::get(), 3u32);

		// Test Rigger In-Servicing MetaData
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let in_service = rigger_details.in_service.get(2).unwrap();
		assert_eq!(*in_service, 2u32);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteInService { who: 1, parachute: reserve_id.clone() }.into(),
		);
	});
}

#[test]
fn rig() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Create SMJ && Loft ID
		// Create Parachutes && Metadata
		let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
		let parachute_owner: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(owner_name).unwrap();

		let drogue_vec: Vec<u8> = "8DG069".as_bytes().to_vec().try_into().unwrap();
		let drogue_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(drogue_vec).unwrap();

		let main_vec: Vec<u8> = "8DC069".as_bytes().to_vec().try_into().unwrap();
		let main_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(main_vec).unwrap();

		let reserve_vec: Vec<u8> = "8RV069".as_bytes().to_vec().try_into().unwrap();
		let reserve_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(reserve_vec).unwrap();

		let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
		let date: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(date_vec).unwrap();

		let loc_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
		let location: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(loc_vec).unwrap();

		assert_ok!(Parachutes::create_loft_id(RuntimeOrigin::signed(1), 1));
		assert_ok!(Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Senior));
		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Drogue,
			parachute_owner.clone(),
			crate::ParachuteModel::Drogue,
			drogue_id.clone(),
			date.clone(),
		));

		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Main,
			parachute_owner.clone(),
			crate::ParachuteModel::DC7,
			main_id.clone(),
			date.clone(),
		));

		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Reserve,
			parachute_owner.clone(),
			crate::ParachuteModel::MT1S,
			reserve_id.clone(),
			date.clone(),
		));

		// Rig Drogue Parachutes
		// Check Drogue Rigging Nonce
		assert_eq!(DroguesRiggedNonce::<Test>::get(), 0);

		// Rig Drogue
		assert_ok!(Parachutes::rig(
			RuntimeOrigin::signed(1),
			1,
			drogue_id.clone(),
			crate::ParachuteType::Drogue,
			None,
			location.clone(),
			date.clone(),
		));

		// Test Drogue Rigging Nonce After Function Call
		assert_eq!(DroguesRiggedNonce::<Test>::get(), 1);

		// Test Rigger Rigging Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let drogue_rigged = rigger_details.drogues_rigged.get(0).unwrap();
		assert_eq!(*drogue_rigged, 0u32);

		// Test Drogue Rigging Metadata
		let drogue_details = Drogues::<Test>::get(drogue_id.clone()).unwrap();
		let rigging_details = drogue_details.rigs.get(0).unwrap();
		assert_eq!(*rigging_details, 0u32);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRigged { who: 1, parachute: drogue_id.clone() }.into(),
		);

		// Rig Main Parachutes
		// Check Main Rigging Nonce
		assert_eq!(MainsRiggedNonce::<Test>::get(), 0);

		// Rig Main
		assert_ok!(Parachutes::rig(
			RuntimeOrigin::signed(1),
			1,
			main_id.clone(),
			crate::ParachuteType::Main,
			Some(drogue_id.clone()),
			location.clone(),
			date.clone(),
		));

		// Test Main Rigging Nonce After Function Call
		assert_eq!(MainsRiggedNonce::<Test>::get(), 1);

		// Test Rigger Rigging Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let main_rigged = rigger_details.mains_rigged.get(0).unwrap();
		assert_eq!(*main_rigged, 0u32);

		// Test Main Parachute Rigging Metadata
		let main_details = Mains::<Test>::get(main_id.clone()).unwrap();
		let rigging_details = main_details.rigs.get(0).unwrap();
		assert_eq!(*rigging_details, 0u32);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRigged { who: 1, parachute: main_id.clone() }.into(),
		);

		// Rig Reserve Parachutes
		// Check Reserve Rigging Nonce
		assert_eq!(ReservesRiggedNonce::<Test>::get(), 0);

		// Rig Reserve
		assert_ok!(Parachutes::rig(
			RuntimeOrigin::signed(1),
			1,
			reserve_id.clone(),
			crate::ParachuteType::Reserve,
			None,
			location.clone(),
			date.clone(),
		));

		// Test Reserve Rigging Nonce After Function Call
		assert_eq!(ReservesRiggedNonce::<Test>::get(), 1);

		// Test Rigger Rigging Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let reserves_rigged = rigger_details.reserves_rigged.get(0).unwrap();
		assert_eq!(*reserves_rigged, 0u32);

		// Test Reserve Rigging Metadata
		let reserve_details = Reserves::<Test>::get(reserve_id.clone()).unwrap();
		let rigging_details = reserve_details.rigs.get(0).unwrap();
		assert_eq!(*rigging_details, 0u32);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRigged { who: 1, parachute: reserve_id.clone() }.into(),
		);
	});
}

#[test]
fn repair() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Create SMJ && Loft ID
		// Create Parachutes && Metadata
		let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
		let parachute_owner: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(owner_name).unwrap();

		let drogue_vec: Vec<u8> = "8DG069".as_bytes().to_vec().try_into().unwrap();
		let drogue_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(drogue_vec).unwrap();

		let main_vec: Vec<u8> = "8DC069".as_bytes().to_vec().try_into().unwrap();
		let main_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(main_vec).unwrap();

		let reserve_vec: Vec<u8> = "8RV069".as_bytes().to_vec().try_into().unwrap();
		let reserve_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(reserve_vec).unwrap();

		let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
		let date: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(date_vec).unwrap();

		let repair_vec: Vec<u8> = "4x4 patch cell 7".as_bytes().to_vec().try_into().unwrap();
		let repair: BoundedVec<u8, <Test as crate::pallet::Config>::MaxRepairLen> =
			BoundedVec::try_from(repair_vec).unwrap();

		assert_ok!(Parachutes::create_loft_id(RuntimeOrigin::signed(1), 1));
		assert_ok!(Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Senior));
		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Drogue,
			parachute_owner.clone(),
			crate::ParachuteModel::Drogue,
			drogue_id.clone(),
			date.clone(),
		));

		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Main,
			parachute_owner.clone(),
			crate::ParachuteModel::DC7,
			main_id.clone(),
			date.clone(),
		));

		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Reserve,
			parachute_owner.clone(),
			crate::ParachuteModel::MT1S,
			reserve_id.clone(),
			date.clone(),
		));

		// Repair Drogue
		// Check Drogue Repair Nonce
		assert_eq!(RepairNonce::<Test>::get(), 0);

		// Repair Drogue
		assert_ok!(Parachutes::repair(
			RuntimeOrigin::signed(1),
			1,
			drogue_id.clone(),
			crate::ParachuteType::Drogue,
			date.clone(),
			repair.clone(),
		));

		// Check Rigger Repairs Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let repairs = rigger_details.repairs.get(0).unwrap();
		assert_eq!(*repairs, 0u32);

		// Check Drogue Repair Metadata
		let drogue_details = Drogues::<Test>::get(drogue_id.clone()).unwrap();
		let drogue_repair = drogue_details.repairs.get(0).unwrap();
		assert_eq!(*drogue_repair, 0u32);

		// Test Repair Nonce After Function Call
		assert_eq!(RepairNonce::<Test>::get(), 1);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRepaired { who: 1, parachute: drogue_id.clone() }.into(),
		);

		// Repair Main
		assert_ok!(Parachutes::repair(
			RuntimeOrigin::signed(1),
			1,
			main_id.clone(),
			crate::ParachuteType::Main,
			date.clone(),
			repair.clone(),
		));

		// Test Rigger Repairs Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let repairs = rigger_details.repairs.get(1).unwrap();
		assert_eq!(*repairs, 1u32);

		// Test Main Repair Metadata
		let main_details = Mains::<Test>::get(main_id.clone()).unwrap();
		let main_repair = main_details.repairs.get(0).unwrap();
		assert_eq!(*main_repair, 1u32);

		// Test Repair Nonce After Function Call
		assert_eq!(RepairNonce::<Test>::get(), 2);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRepaired { who: 1, parachute: main_id.clone() }.into(),
		);

		// Repair Reserve
		assert_ok!(Parachutes::repair(
			RuntimeOrigin::signed(1),
			1,
			reserve_id.clone(),
			crate::ParachuteType::Reserve,
			date.clone(),
			repair.clone(),
		));

		// Test Rigger Repairs Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let repairs = rigger_details.repairs.get(2).unwrap();
		assert_eq!(*repairs, 2u32);

		// Test Reserve Repair Metadata
		let reserve_details = Reserves::<Test>::get(reserve_id.clone()).unwrap();
		let reserve_repair = reserve_details.repairs.get(0).unwrap();
		assert_eq!(*reserve_repair, 2u32);

		// Test Repair Nonce After Function Call
		assert_eq!(RepairNonce::<Test>::get(), 3);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRepaired { who: 1, parachute: reserve_id.clone() }.into(),
		);
	});
}

#[test]
fn retire() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Create SMJ && Loft ID
		// Create Parachutes && Metadata
		let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
		let parachute_owner: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(owner_name).unwrap();

		let drogue_vec: Vec<u8> = "8DG069".as_bytes().to_vec().try_into().unwrap();
		let drogue_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(drogue_vec).unwrap();

		let drogue_vec2: Vec<u8> = "8DG420".as_bytes().to_vec().try_into().unwrap();
		let drogue_id2: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(drogue_vec2).unwrap();

		let main_vec: Vec<u8> = "8DC069".as_bytes().to_vec().try_into().unwrap();
		let main_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(main_vec).unwrap();

		let reserve_vec: Vec<u8> = "8RV069".as_bytes().to_vec().try_into().unwrap();
		let reserve_id: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(reserve_vec).unwrap();

		let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
		let date: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(date_vec).unwrap();

		let loc_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
		let location: BoundedVec<u8, <Test as crate::pallet::Config>::MaxIdLen> =
			BoundedVec::try_from(loc_vec).unwrap();

		let repair_vec: Vec<u8> = "4x4 patch cell 7".as_bytes().to_vec().try_into().unwrap();
		let repair: BoundedVec<u8, <Test as crate::pallet::Config>::MaxRepairLen> =
			BoundedVec::try_from(repair_vec).unwrap();

		assert_ok!(Parachutes::create_loft_id(RuntimeOrigin::signed(1), 1));
		assert_ok!(Parachutes::set_rigger(RuntimeOrigin::signed(1), 1, crate::Rigger::Senior));
		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Drogue,
			parachute_owner.clone(),
			crate::ParachuteModel::Drogue,
			drogue_id.clone(),
			date.clone(),
		));

		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Drogue,
			parachute_owner.clone(),
			crate::ParachuteModel::Drogue,
			drogue_id2.clone(),
			date.clone(),
		));

		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Main,
			parachute_owner.clone(),
			crate::ParachuteModel::DC7,
			main_id.clone(),
			date.clone(),
		));

		assert_ok!(Parachutes::in_service(
			RuntimeOrigin::signed(1),
			1,
			crate::ParachuteType::Reserve,
			parachute_owner.clone(),
			crate::ParachuteModel::MT1S,
			reserve_id.clone(),
			date.clone(),
		));

		// Check Retire Nonce
		assert_eq!(RetireNonce::<Test>::get(), 0);

		// Retire Drogue
		assert_ok!(Parachutes::retire(
			RuntimeOrigin::signed(1),
			1,
			drogue_id.clone(),
			crate::ParachuteType::Drogue,
			date.clone(),
		));

		// Test Drogue Metadata
		let drogue_details = Drogues::<Test>::get(drogue_id.clone()).unwrap();
		assert_eq!(drogue_details.retired, Some(0u32));
		assert_eq!(drogue_details.active, false);

		// Test Rigger Retires Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let retires = rigger_details.retires.get(0).unwrap();
		assert_eq!(*retires, 0u32);

		// Test Retire Nonce After Function Call
		assert_eq!(RetireNonce::<Test>::get(), 1);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRetired { who: 1, parachute: drogue_id.clone() }.into(),
		);

		// Test Rig Retired Drogue
		assert_noop!(
			Parachutes::rig(
				RuntimeOrigin::signed(1),
				1,
				drogue_id.clone(),
				crate::ParachuteType::Drogue,
				None,
				location.clone(),
				date.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Test Repair Retired Drogue
		assert_noop!(
			Parachutes::repair(
				RuntimeOrigin::signed(1),
				1,
				drogue_id.clone(),
				crate::ParachuteType::Drogue,
				date.clone(),
				repair.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Test Retire Retired Drogue
		assert_noop!(
			Parachutes::retire(
				RuntimeOrigin::signed(1),
				1,
				drogue_id.clone(),
				crate::ParachuteType::Drogue,
				date.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Test Rig Main with Retired Drogue
		assert_noop!(
			Parachutes::rig(
				RuntimeOrigin::signed(1),
				1,
				main_id.clone(),
				crate::ParachuteType::Main,
				Some(drogue_id.clone()),
				location.clone(),
				date.clone(),
			),
			Error::<Test>::DrogueRetired
		);

		// Retire Main
		assert_ok!(Parachutes::retire(
			RuntimeOrigin::signed(1),
			1,
			main_id.clone(),
			crate::ParachuteType::Main,
			date.clone(),
		));

		// Test Main Metadata
		let main_details = Mains::<Test>::get(main_id.clone()).unwrap();
		assert_eq!(main_details.retired, Some(1u32));
		assert_eq!(main_details.active, false);

		// Test Rigger Retires Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let retires = rigger_details.retires.get(1).unwrap();
		assert_eq!(*retires, 1u32);

		// Test Retire Nonce After Function Call
		assert_eq!(RetireNonce::<Test>::get(), 2);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRetired { who: 1, parachute: main_id.clone() }.into(),
		);

		// Test Rig Retired Main
		assert_noop!(
			Parachutes::rig(
				RuntimeOrigin::signed(1),
				1,
				main_id.clone(),
				crate::ParachuteType::Main,
				Some(drogue_id2.clone()),
				location.clone(),
				date.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Test Repair Retired Main
		assert_noop!(
			Parachutes::repair(
				RuntimeOrigin::signed(1),
				1,
				main_id.clone(),
				crate::ParachuteType::Main,
				date.clone(),
				repair.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Test Retire Retired Main
		assert_noop!(
			Parachutes::retire(
				RuntimeOrigin::signed(1),
				1,
				main_id.clone(),
				crate::ParachuteType::Main,
				date.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Retire Reserve
		assert_ok!(Parachutes::retire(
			RuntimeOrigin::signed(1),
			1,
			reserve_id.clone(),
			crate::ParachuteType::Reserve,
			date.clone(),
		));

		// Test Reserve Metadata
		let reserve_details = Reserves::<Test>::get(reserve_id.clone()).unwrap();
		assert_eq!(reserve_details.retired, Some(2u32));
		assert_eq!(reserve_details.active, false);

		// Test Rigger Retires Metadata
		let rigger_details = Riggers::<Test>::get(1).unwrap();
		let retires = rigger_details.retires.get(2).unwrap();
		assert_eq!(*retires, 2u32);

		// Test Retires Nonce After Function Call
		assert_eq!(RetireNonce::<Test>::get(), 3);

		// Last Event
		System::assert_last_event(
			Event::<Test>::ParachuteRetired { who: 1, parachute: reserve_id.clone() }.into(),
		);

		// Test Rig Retired Reserve
		assert_noop!(
			Parachutes::rig(
				RuntimeOrigin::signed(1),
				1,
				reserve_id.clone(),
				crate::ParachuteType::Reserve,
				None,
				location.clone(),
				date.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Test Repair Retire Reserve
		assert_noop!(
			Parachutes::repair(
				RuntimeOrigin::signed(1),
				1,
				reserve_id.clone(),
				crate::ParachuteType::Reserve,
				date.clone(),
				repair.clone(),
			),
			Error::<Test>::ParachuteRetired
		);

		// Test Retire Retired Reserve
		assert_noop!(
			Parachutes::retire(
				RuntimeOrigin::signed(1),
				1,
				reserve_id.clone(),
				crate::ParachuteType::Reserve,
				date.clone(),
			),
			Error::<Test>::ParachuteRetired
		);
	});
}

// TODO: FIX TYPING - EXAMPLE OF WHATS WRONG
// let retire_details =
//     Retire::<Test>::get(0)
//        .ok_or(Error::<Test>::NoDetails).unwrap();
// assert_eq!(
//        retire_details,
//        { RetireDetails {
//            who: 1,
//            parachute: drogue_id.clone(),
//            date: date.clone()}
//        }.into()        );
