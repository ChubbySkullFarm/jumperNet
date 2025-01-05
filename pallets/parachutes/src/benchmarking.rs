#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Parachutes;

use frame_benchmarking::v2::*;
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use scale_info::prelude::vec::Vec;

const SEED: u32 = 0;
#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_loft_id<T: Config>() {
		// Set initial conditions
		let caller: T::AccountId = account("BenchMark", 1, SEED);

		// Call extrinsic
		#[extrinsic_call]
		create_loft_id(RawOrigin::Signed(caller.clone()), caller.clone());

		// Verify
		assert!(Riggers::<T>::contains_key(caller));
	}

	#[benchmark]
	fn set_rigger<T: Config>() {
		// Set initial conditions
		let caller: T::AccountId = account("BenchMark", 1, SEED);
		let _ =
			Pallet::<T>::create_loft_id(RawOrigin::Signed(caller.clone()).into(), caller.clone());

		// Call Extrinsic
		#[extrinsic_call]
		set_rigger(RawOrigin::Signed(caller.clone()), caller.clone(), Rigger::Senior);

		// Verify
		let smj_data = Riggers::<T>::get(caller.clone()).unwrap();
		assert_eq!(smj_data.rigger, Rigger::Senior);
	}

	#[benchmark]
	fn in_service<T: Config>() {
		// Set initial conditions
		// Create Smokejumper
		let caller: T::AccountId = account("BenchMark", 1, SEED);
		let _ =
			Pallet::<T>::create_loft_id(RawOrigin::Signed(caller.clone()).into(), caller.clone());
		let _ = Pallet::<T>::set_rigger(
			RawOrigin::Signed(caller.clone()).into(),
			caller.clone(),
			Rigger::Senior,
		);

		// Create Parachute
		let drogue_vec: Vec<u8> = "8DG069".as_bytes().to_vec().try_into().unwrap();
		let drogue_id: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(drogue_vec).unwrap();

		// Create Metadata
		let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
		let parachute_owner: BoundedVec<u8, T::MaxIdLen> =
			BoundedVec::try_from(owner_name).unwrap();

		let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
		let date: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(date_vec).unwrap();

		// Call Extrinsic
		#[extrinsic_call]
		in_service(
			RawOrigin::Signed(caller.clone()),
			caller.clone(),
			ParachuteType::Drogue,
			parachute_owner,
			ParachuteModel::Drogue,
			drogue_id.clone(),
			date,
		);

		// Verify
		assert!(Drogues::<T>::contains_key(drogue_id));
	}

    #[benchmark]
    fn rig<T: Config>() {
        // Rig Main Parachute chosen for weight because of drogue check
        // Create Smokejumper
        let caller: T::AccountId = account("BenchMark", 1, SEED);
        let _ =
            Pallet::<T>::create_loft_id(RawOrigin::Signed(caller.clone()).into(), caller.clone());
        let _ = Pallet::<T>::set_rigger(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            Rigger::Senior,
        );

        // Create Drogue Parachute Metadata
        let drogue_vec: Vec<u8> = "8DG069".as_bytes().to_vec().try_into().unwrap();
        let drogue_id: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(drogue_vec).unwrap();

        // Create Main Parachute Metadata
        let main_vec: Vec<u8> = "8DC420".as_bytes().to_vec().try_into().unwrap();
        let main_id: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(main_vec).unwrap();

        let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
        let parachute_owner: BoundedVec<u8, T::MaxIdLen> =
            BoundedVec::try_from(owner_name).unwrap();

        let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
        let date: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(date_vec).unwrap();

        // Create Drogue
        let _ = Pallet::<T>::in_service(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            ParachuteType::Drogue,
            parachute_owner.clone(),
            ParachuteModel::Drogue,
            drogue_id.clone(),
            date.clone(),
        );

        // Create Main
        let _ = Pallet::<T>::in_service(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            ParachuteType::Main,
            parachute_owner,
            ParachuteModel::DC7,
            main_id.clone(),
            date.clone(),
        );

        // more metadata
        let location_vec: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
        let location: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(location_vec).unwrap();

        // Call Extrinsic
        #[extrinsic_call]
        rig(
            RawOrigin::Signed(caller.clone()),
            caller.clone(),
            main_id.clone(),
            ParachuteType::Main,
            Some(drogue_id.clone()),
            location,
            date,
        );

        // Verify
        assert!(MainsRigged::<T>::contains_key(0u32));
    }

    #[benchmark]
    fn repair_weight<T: Config>() {
        // Create Smokejumper
        let caller: T::AccountId = account("BenchMark", 1, SEED);
        let _ =
            Pallet::<T>::create_loft_id(RawOrigin::Signed(caller.clone()).into(), caller.clone());
        let _ = Pallet::<T>::set_rigger(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            Rigger::Senior,
        );
    
        // Create Parachute
        let reserve_vec: Vec<u8> = "8RV069".as_bytes().to_vec().try_into().unwrap();
        let reserve_id: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(reserve_vec).unwrap();
   
        let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
        let parachute_owner: BoundedVec<u8, T::MaxIdLen> =
            BoundedVec::try_from(owner_name).unwrap();
   
        // Meatadata
        let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
        let date: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(date_vec).unwrap();
   
        let repair_vec: Vec<u8> = "4x4 hole cell 6 tail".as_bytes().to_vec().try_into().unwrap();
        let repair: BoundedVec<u8, T::MaxRepairLen> = BoundedVec::try_from(repair_vec).unwrap();
   
        let _ = Pallet::<T>::in_service(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            ParachuteType::Reserve,
            parachute_owner,
            ParachuteModel::MT1S,
            reserve_id.clone(),
            date.clone(),
        );

        // Call Extrinsic
        #[extrinsic_call]
        repair(
            RawOrigin::Signed(caller.clone()),
            caller.clone(),
            reserve_id.clone(),
            ParachuteType::Reserve,
            date,
            repair.clone(),
        );

        // Verify
        assert!(Repair::<T>::contains_key(0u32));
    }

    #[benchmark]
    fn retire<T: Config>() {
        // Create Smokejumper
        let caller: T::AccountId = account("BenchMark", 1, SEED);
        let _ =
            Pallet::<T>::create_loft_id(RawOrigin::Signed(caller.clone()).into(), caller.clone());
        let _ = Pallet::<T>::set_rigger(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            Rigger::Senior,
        );

        // Create Parachute
        let drogue_vec: Vec<u8> = "8DG069".as_bytes().to_vec().try_into().unwrap();
        let drogue_id: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(drogue_vec).unwrap();

        let owner_name: Vec<u8> = "AK".as_bytes().to_vec().try_into().unwrap();
        let parachute_owner: BoundedVec<u8, T::MaxIdLen> =
            BoundedVec::try_from(owner_name).unwrap();

        // Meatadata
        let date_vec: Vec<u8> = "04/20/1969".as_bytes().to_vec().try_into().unwrap();
        let date: BoundedVec<u8, T::MaxIdLen> = BoundedVec::try_from(date_vec).unwrap();

        let _ = Pallet::<T>::in_service(
            RawOrigin::Signed(caller.clone()).into(),
            caller.clone(),
            ParachuteType::Drogue,
            parachute_owner,
            ParachuteModel::Drogue,
            drogue_id.clone(),
            date.clone(),
        );

        // Call Extrinsic
        #[extrinsic_call]
        retire(
            RawOrigin::Signed(caller.clone()),
            caller.clone(),
            drogue_id.clone(),
            ParachuteType::Drogue,
            date,
        );

        // Verify
        let parachute_data = Drogues::<T>::get(drogue_id.clone()).unwrap();
        assert_eq!(parachute_data.active, false);
    }

	impl_benchmark_test_suite!(Parachutes, crate::mock::new_test_ext(), crate::mock::Test);
}
