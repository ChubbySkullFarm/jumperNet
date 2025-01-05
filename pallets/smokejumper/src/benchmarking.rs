#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Smokejumper;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{BoundedVec};
use scale_info::prelude::vec::Vec;

const SEED: u32 = 0;
const SEED2: u32 = 1;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_smj<T: Config>() {
        // Set initial conditions
        
        let caller: T::AccountId = account("BenchMark", 1, SEED);
        let smj_name: Vec<u8> = "Ernie".as_bytes().to_vec().try_into().unwrap();
        let bounded_name: BoundedVec<u8, T::MaxLength> =
            BoundedVec::try_from(smj_name).unwrap();

        // Call extrinsic
		#[extrinsic_call]
        create_smj(RawOrigin::Signed(caller.clone()), caller.clone(), bounded_name);

        // Verify
        assert!(Smokejumpers::<T>::contains_key(caller));
	}

    #[benchmark]
    fn change_name<T: Config>() {
        // Set intitial conditions
        // Create New Smokejumper
        let caller: T::AccountId = account("BenchMark", 1, SEED);
        let smj_name: Vec<u8> = "Ernie".as_bytes().to_vec().try_into().unwrap();
        let bounded_name: BoundedVec<u8, T::MaxLength> =
            BoundedVec::try_from(smj_name).unwrap();
        let _ = Pallet::<T>::create_smj(RawOrigin::Signed(caller.clone()).into(), caller.clone(), bounded_name);

        // Create new name
        let new_name: Vec<u8> = "Skeebo".as_bytes().to_vec().try_into().unwrap();
        let new_bounded: BoundedVec<u8, T::MaxLength> =
            BoundedVec::try_from(new_name).unwrap();

        // Call Extrinsic
        #[extrinsic_call]
        change_name(RawOrigin::Signed(caller.clone()), caller.clone(), new_bounded.clone());

        // Verify
        let new_data = Smokejumpers::<T>::get(caller.clone()).ok_or(Error::<T>::SmjDoesNotExist).unwrap();
        assert_eq!(new_data.name, new_bounded);
    }

    #[benchmark]
    fn swap_address<T: Config>() {
        // Set initial conditions
        // Create new Smokejumper
        let caller: T::AccountId = account("BenchMark", 1, SEED);
        let smj_name: Vec<u8> = "Ernie".as_bytes().to_vec().try_into().unwrap();
        let bounded_name: BoundedVec<u8, T::MaxLength> =
            BoundedVec::try_from(smj_name).unwrap();
        let _ = Pallet::<T>::create_smj(RawOrigin::Signed(caller.clone()).into(), caller.clone(), bounded_name);

        let new_address: T::AccountId = account("NEW", 2, SEED2);

        // Call Extrinsic
        #[extrinsic_call]
        swap_address(RawOrigin::Signed(caller.clone()), caller.clone(), new_address.clone());

        // Verify
        assert!(Smokejumpers::<T>::contains_key(new_address));
    }


	impl_benchmark_test_suite!(Smokejumper, crate::mock::new_test_ext(), crate::mock::Test);
}
