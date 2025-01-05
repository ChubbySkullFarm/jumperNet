#![cfg_attr(not(feature = "std"), no_std)]

//TODO:
// WEIGHTS
// BENCHMARKING
// append() instead of push()?
// Work on properly defining types with traits
//    - Move to separate module
//    - Metadata structs need help
//    - impls for new() && || default()
// Move Storage to separate module?
//    - simplify?
// Enums???

pub use pallet::*;

mod impls;
pub mod weights;
use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


pub use pallet_smokejumper;
use scale_info::prelude::fmt::Debug;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type MaxIdLen: Get<u32>;
		type MaxRigs: Get<u32>;
		type MaxRepairLen: Get<u32>;
	}

	pub type ParachuteId<T> = BoundedVec<u8, <T as pallet::Config>::MaxIdLen>;

	pub type RigNonce = u32;

	// Loft metadata attached to SMJ
	#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct SmjLoftDetails<T: Config> {
		pub rigger: Rigger,
		pub in_service: BoundedVec<u32, T::MaxRigs>,
		pub drogues_rigged: BoundedVec<u32, T::MaxRigs>,
		pub mains_rigged: BoundedVec<u32, T::MaxRigs>,
		pub reserves_rigged: BoundedVec<u32, T::MaxRigs>,
		pub repairs: BoundedVec<u32, T::MaxRigs>,
		pub retires: BoundedVec<u32, T::MaxRigs>,
	}

	// Lifecycle metadata for Parachutes
	#[derive(Clone, Decode, Encode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ParachuteDetails<T: Config> {
		pub active: bool,
		pub owner: BoundedVec<u8, T::MaxIdLen>,
		pub model: ParachuteModel,
		pub in_service: u32,
		pub rigs: BoundedVec<u32, T::MaxRigs>,
		pub repairs: BoundedVec<u32, T::MaxRigs>,
		pub retired: Option<u32>,
	}

	// In Service metadata
	#[derive(Clone, Decode, Encode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct InServiceDetails<T: Config> {
		pub who: T::AccountId,
		pub parachute: ParachuteId<T>,
		pub date: BoundedVec<u8, T::MaxIdLen>,
	}

	// Rigging metadata
	#[derive(Clone, Decode, Encode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct RiggingDetails<T: Config> {
		pub who: T::AccountId,
		pub parachute: ParachuteId<T>,
		pub drogue: Option<ParachuteId<T>>,
		pub location: BoundedVec<u8, T::MaxIdLen>,
		pub date: BoundedVec<u8, T::MaxIdLen>,
	}

	// Repair metadata
	#[derive(Clone, Decode, Encode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct RepairDetails<T: Config> {
		pub who: T::AccountId,
		pub parachute: ParachuteId<T>,
		pub date: BoundedVec<u8, T::MaxIdLen>,
		pub repair: BoundedVec<u8, T::MaxRepairLen>,
	}

	// Retire metadata
	#[derive(Clone, Decode, Encode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct RetireDetails<T: Config> {
		pub who: T::AccountId,
		pub parachute: ParachuteId<T>,
		pub date: BoundedVec<u8, T::MaxIdLen>,
	}

	// FAA Parachute Rigger Certifications
	#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub enum Rigger {
		Uncertified,
		Senior,
		Master,
	}

	// Types of Parachutes
	#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub enum ParachuteType {
		Drogue,
		Main,
		Reserve,
	}

	// Parachute Models
	#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub enum ParachuteModel {
		Drogue,
		DC7,
		CR360,
		MT1S,
	}

	// Index SMJ to Loft metadata
	#[pallet::storage]
	pub(super) type Riggers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, SmjLoftDetails<T>>;

	// In Service Ledger
	#[pallet::storage]
	pub(super) type InService<T: Config> =
		StorageMap<_, Blake2_128Concat, RigNonce, InServiceDetails<T>>;

	// Nonce to keep track of each in service - provide index/key
	#[pallet::storage]
	pub(super) type InServiceNonce<T: Config> = StorageValue<_, RigNonce, ValueQuery>;

	// Repairs ledger
	#[pallet::storage]
	pub(super) type Repair<T: Config> = StorageMap<_, Blake2_128Concat, RigNonce, RepairDetails<T>>;

	// Nonce to keep each track of each repair - provide index/key
	#[pallet::storage]
	pub(super) type RepairNonce<T: Config> = StorageValue<_, RigNonce, ValueQuery>;

	// Retire ledger
	#[pallet::storage]
	pub(super) type Retire<T: Config> = StorageMap<_, Blake2_128Concat, RigNonce, RetireDetails<T>>;

	// Nonce for Retire leger - provide index/key
	#[pallet::storage]
	pub(super) type RetireNonce<T: Config> = StorageValue<_, RigNonce, ValueQuery>;

	// Drogue Parachutes
	#[pallet::storage]
	pub(super) type Drogues<T: Config> =
		StorageMap<_, Blake2_128Concat, ParachuteId<T>, ParachuteDetails<T>>;

	// Rigging Ledger for Drogues
	#[pallet::storage]
	pub(super) type DroguesRigged<T: Config> =
		StorageMap<_, Blake2_128Concat, RigNonce, RiggingDetails<T>>;

	// Nonce for Drogue Rigging Ledger - provide index/key
	#[pallet::storage]
	pub(super) type DroguesRiggedNonce<T: Config> = StorageValue<_, RigNonce, ValueQuery>;

	// Main Parachutes
	#[pallet::storage]
	pub(super) type Mains<T: Config> =
		StorageMap<_, Blake2_128Concat, ParachuteId<T>, ParachuteDetails<T>>;

	// Rigging Ledger for Main Parachutes
	#[pallet::storage]
	pub(super) type MainsRigged<T: Config> =
		StorageMap<_, Blake2_128Concat, RigNonce, RiggingDetails<T>>;

	// Nonce for Main Rigging Ledger - provide index/key
	#[pallet::storage]
	pub(super) type MainsRiggedNonce<T: Config> = StorageValue<_, RigNonce, ValueQuery>;

	// Reserve Parachutes
	#[pallet::storage]
	pub(super) type Reserves<T: Config> =
		StorageMap<_, Blake2_128Concat, ParachuteId<T>, ParachuteDetails<T>>;

	// Rigging Ledger for Reserves
	#[pallet::storage]
	pub(super) type ReservesRigged<T: Config> =
		StorageMap<_, Blake2_128Concat, RigNonce, RiggingDetails<T>>;

	// Nonce for Reserves rigged - provide index/key
	#[pallet::storage]
	pub(super) type ReservesRiggedNonce<T: Config> = StorageValue<_, RigNonce, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewRigger { who: T::AccountId },
		ParachuteInService { who: T::AccountId, parachute: ParachuteId<T> },
		ParachuteRigged { who: T::AccountId, parachute: ParachuteId<T> },
		ParachuteRepaired { who: T::AccountId, parachute: ParachuteId<T> },
		ParachuteRetired { who: T::AccountId, parachute: ParachuteId<T> },
		RiggerChanged { who: T::AccountId, rigger: Rigger },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyARigger,
		DrogueDoesNotExist,
		DrogueRetired,
		NoDrogue,
		NoDetails,
		NoLoftId,
		NotARigger,
		NotQualified,
		ParachuteAlreadyExists,
		ParachuteRetired,
		ParachuteDoesNotExist,
		SmjDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_loft_id())]
		pub fn create_loft_id(origin: OriginFor<T>, new_smj: T::AccountId) -> DispatchResult {
			let _qualified = ensure_signed(origin)?;
			Self::do_create_loft_id(new_smj)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::set_rigger())]
		pub fn set_rigger(
			origin: OriginFor<T>,
			smj: T::AccountId,
			rigger: Rigger,
		) -> DispatchResult {
			let _qualified = ensure_signed(origin)?;
			Self::do_set_rigger(smj, rigger)?;
			Ok(())
		}

		#[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::in_service())]
		pub fn in_service(
			origin: OriginFor<T>,
			rigger: T::AccountId,
			parachute_type: ParachuteType,
			parachute_owner: BoundedVec<u8, T::MaxIdLen>,
			parachute_model: ParachuteModel,
			id: ParachuteId<T>,
			date: BoundedVec<u8, T::MaxIdLen>,
		) -> DispatchResult {
			let _qualified = ensure_signed(origin)?;
			Self::do_in_service(
				rigger,
				parachute_type,
				parachute_owner,
				parachute_model,
				id,
				date,
			)?;
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::rig())]
		pub fn rig(
			origin: OriginFor<T>,
			rigger: T::AccountId,
			parachute: ParachuteId<T>,
			parachute_type: ParachuteType,
			drogue: Option<ParachuteId<T>>,
			location: BoundedVec<u8, T::MaxIdLen>,
			date: BoundedVec<u8, T::MaxIdLen>,
		) -> DispatchResult {
			let _qualified = ensure_signed(origin)?;
			Self::do_rig(rigger, parachute, parachute_type, drogue, location, date)?;
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::repair_weight())]
		pub fn repair(
			origin: OriginFor<T>,
			rigger: T::AccountId,
			parachute: ParachuteId<T>,
			parachute_type: ParachuteType,
			date: BoundedVec<u8, T::MaxIdLen>,
			repair: BoundedVec<u8, T::MaxRepairLen>,
		) -> DispatchResult {
			let _qualified = ensure_signed(origin)?;
			Self::do_repair(rigger, parachute, parachute_type, date, repair)?;
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::retire())]
		pub fn retire(
			origin: OriginFor<T>,
			rigger: T::AccountId,
			parachute: ParachuteId<T>,
			parachute_type: ParachuteType,
			date: BoundedVec<u8, T::MaxIdLen>,
		) -> DispatchResult {
			let _qualified = ensure_signed(origin)?;
			Self::do_retire(rigger, parachute, parachute_type, date)?;
			Ok(())
		}
	}
}
