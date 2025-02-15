#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

mod impls;
pub mod weights;
use weights::WeightInfo;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{
	sp_runtime::SaturatedConversion,
	traits::tokens::fungible::{Balanced, Inspect, Mutate},
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type MaxLength: Get<u32>;
		type NativeBalance: Inspect<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ Balanced<Self::AccountId>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Encode, Debug, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Smokejumper<T: Config> {
		pub name: BoundedVec<u8, T::MaxLength>,
	}

	#[pallet::storage]
	#[pallet::getter(fn smokejumpers)]
	pub(super) type Smokejumpers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Smokejumper<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IdSwap { smj: BoundedVec<u8, T::MaxLength> },
		NameChanged { smj: BoundedVec<u8, T::MaxLength> },
		SmjCreated { smj: BoundedVec<u8, T::MaxLength> },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadySmj,
		SmjAlreadyCreated,
		SmjDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_smj())]
		pub fn create_smj(
			origin: OriginFor<T>,
			new_smj: T::AccountId,
			name: BoundedVec<u8, T::MaxLength>,
		) -> DispatchResult {
			let _old_smj = ensure_signed(origin)?;
			Self::do_create_smj(new_smj, name)?;
			Ok(())
		}

		#[pallet::call_index(1)]
        	#[pallet::weight(T::WeightInfo::change_name())]
		pub fn change_name(
			origin: OriginFor<T>,
			smj: T::AccountId,
			name: BoundedVec<u8, T::MaxLength>,
		) -> DispatchResult {
			let _old_smj = ensure_signed(origin)?;
			Self::do_change_name(smj, name)?;
			Ok(())
		}

		#[pallet::call_index(2)]
        	#[pallet::weight(T::WeightInfo::swap_address())]
		pub fn swap_address(
			origin: OriginFor<T>,
			smj: T::AccountId,
			new_id: T::AccountId,
		) -> DispatchResult {
			let _old_smj = ensure_signed(origin)?;
			Self::do_swap_address(smj, new_id)?;
			Ok(())
		}
	}
}
