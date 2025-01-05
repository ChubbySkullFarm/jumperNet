use super::*;
use frame_support::{pallet_prelude::*, BoundedVec};

//TODO: Build functions to remove repetative code

impl<T: Config> Pallet<T> {
	// Create Loft ID for SMJ
	pub fn do_create_loft_id(new_smj: T::AccountId) -> DispatchResult {
		// Ensure ID not already creates
		ensure!(!Riggers::<T>::contains_key(new_smj.clone()), Error::<T>::AlreadyARigger);

		// Build new details
		let empty_vec: BoundedVec<u32, T::MaxRigs> = BoundedVec::new();

		let loft_data = SmjLoftDetails {
			rigger: Rigger::Uncertified,
			in_service: empty_vec.clone(),
			drogues_rigged: empty_vec.clone(),
			mains_rigged: empty_vec.clone(),
			reserves_rigged: empty_vec.clone(),
			repairs: empty_vec.clone(),
			retires: empty_vec,
		};

		// Insert into storage
		Riggers::<T>::insert(new_smj.clone(), loft_data);
		Self::deposit_event(Event::<T>::NewRigger { who: new_smj });
		Ok(())
	}

	// Set Rigger status of SMJ
	pub fn do_set_rigger(smj: T::AccountId, rigger: Rigger) -> DispatchResult {
		// Check for Loft ID
		let mut rigger_data: SmjLoftDetails<T> =
			Riggers::<T>::get(smj.clone()).ok_or(Error::<T>::NoLoftId)?;
		ensure!(rigger_data.rigger != rigger.clone(), Error::<T>::AlreadyARigger);
		rigger_data.rigger = rigger.clone();

		// Change Rigger status in storage
		Riggers::<T>::set(smj.clone(), Some(rigger_data));

		Self::deposit_event(Event::<T>::RiggerChanged { who: smj, rigger });
		Ok(())
	}

	// Create A New Parachute
	pub fn do_in_service(
		rigger: T::AccountId,
		parachute_type: ParachuteType,
		parachute_owner: BoundedVec<u8, T::MaxIdLen>,
		parachute_model: ParachuteModel,
		id: ParachuteId<T>,
		date: BoundedVec<u8, T::MaxIdLen>,
	) -> DispatchResult {
		// Check qualifications
		let mut smj = Riggers::<T>::get(rigger.clone()).ok_or(Error::<T>::SmjDoesNotExist).unwrap();
		ensure!(
			smj.rigger == Rigger::Senior || smj.rigger == Rigger::Master,
			Error::<T>::NotQualified
		);

		// Ensure parachute doesnt exist
		match parachute_type {
			ParachuteType::Drogue => {
				ensure!(
					!Drogues::<T>::contains_key(id.clone()),
					Error::<T>::ParachuteAlreadyExists
				);
			},
			ParachuteType::Main => {
				ensure!(!Mains::<T>::contains_key(id.clone()), Error::<T>::ParachuteAlreadyExists);
			},
			ParachuteType::Reserve => {
				ensure!(
					!Reserves::<T>::contains_key(id.clone()),
					Error::<T>::ParachuteAlreadyExists
				);
			},
		}

		// Get nonce
		let mut nonce = InServiceNonce::<T>::get();

		// Create Parachute Matadata
		let empty_vec: BoundedVec<u32, T::MaxRigs> = BoundedVec::new();

		let new_parachute = ParachuteDetails {
			active: true,
			owner: parachute_owner,
			model: parachute_model,
			in_service: nonce.clone(),
			rigs: empty_vec.clone(),
			repairs: empty_vec,
			retired: None,
		};

		// Match and in service (set)
		match parachute_type {
			ParachuteType::Drogue => Drogues::<T>::set(id.clone(), Some(new_parachute)),
			ParachuteType::Main => Mains::<T>::set(id.clone(), Some(new_parachute)),
			ParachuteType::Reserve => Reserves::<T>::set(id.clone(), Some(new_parachute)),
		};

		// Create metadata for in servicing
		let new_in_service = InServiceDetails { who: rigger.clone(), parachute: id.clone(), date };

		// Insert in servicing into storage
		InService::<T>::set(nonce.clone(), Some(new_in_service));

		// Push nonce to SMJ Loft Metadata
		let mut new_in_service = smj.in_service.clone().to_vec();
		new_in_service.push(nonce.clone());
		smj.in_service = BoundedVec::try_from(new_in_service).unwrap();
		Riggers::<T>::set(rigger.clone(), Some(smj));

		// Increase nonce for next in service
		nonce = nonce.saturating_add(1);
		InServiceNonce::<T>::set(nonce);

		Self::deposit_event(Event::<T>::ParachuteInService { who: rigger, parachute: id });

		Ok(())
	}

	// Rig Parachutes
	pub fn do_rig(
		rigger: T::AccountId,
		parachute: ParachuteId<T>,
		parachute_type: ParachuteType,
		drogue: Option<ParachuteId<T>>,
		location: BoundedVec<u8, T::MaxIdLen>,
		date: BoundedVec<u8, T::MaxIdLen>,
	) -> DispatchResult {
		let mut smj = Riggers::<T>::get(rigger.clone()).ok_or(Error::<T>::SmjDoesNotExist).unwrap();

		match parachute_type {
			// Rig a Drogue
			ParachuteType::Drogue => {
				// Ensure drogue exists & active
				let mut parachute_data: ParachuteDetails<T> = Drogues::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get nonce
				let nonce = DroguesRiggedNonce::<T>::get();

				// Create Rigging Metadata
				let new_details = RiggingDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					drogue: None,
					location,
					date,
				};

				// Insert into Rigging metadata into storage
				DroguesRigged::<T>::set(nonce.clone(), Some(new_details));

				// Push nonce to Parachute Metadata
				let mut new_rigs = parachute_data.rigs.clone().to_vec();
				new_rigs.push(nonce.clone());
				parachute_data.rigs = BoundedVec::try_from(new_rigs).unwrap();
				Drogues::<T>::set(parachute.clone(), Some(parachute_data));

				// Push nonce to SMJ Loft metadata
				let mut new_smj_rigs = smj.drogues_rigged.clone().to_vec();
				new_smj_rigs.push(nonce.clone());
				smj.drogues_rigged = BoundedVec::try_from(new_smj_rigs).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				let nonce = nonce.saturating_add(1);
				DroguesRiggedNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRigged { who: rigger, parachute });
			},
			// Rig a Main Parachute
			ParachuteType::Main => {
				// Ensure a drogue is included and exists
				ensure!(drogue.clone() == Some(drogue.clone()).unwrap(), Error::<T>::NoDrogue);
				let drogue = drogue.unwrap();
				let drogue_data: ParachuteDetails<T> =
					Drogues::<T>::get(drogue.clone()).ok_or(Error::<T>::DrogueDoesNotExist)?;
				ensure!(drogue_data.active == true, Error::<T>::DrogueRetired);

				// Ensure Main parachute exists & active
				let mut parachute_data = Mains::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let nonce = MainsRiggedNonce::<T>::get();

				// Create Rigging Metadata
				let new_details = RiggingDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					drogue: Some(drogue.clone()),
					location,
					date,
				};

				// Insert Rigging Metadata into Storage
				MainsRigged::<T>::set(nonce.clone(), Some(new_details));

				// Push Nonce to Main Parachute Metadata
				let mut new_rigs = parachute_data.rigs.clone().to_vec();
				new_rigs.push(nonce.clone());
				parachute_data.rigs = BoundedVec::try_from(new_rigs).unwrap();
				Mains::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Loft Metadata
				let mut new_smj_rigs = smj.mains_rigged.clone().to_vec();
				new_smj_rigs.push(nonce.clone());
				smj.mains_rigged = BoundedVec::try_from(new_smj_rigs).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				let nonce = nonce.saturating_add(1);
				MainsRiggedNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRigged { who: rigger, parachute });
			},
			ParachuteType::Reserve => {
				// Ensure Rigger is Qualified
				ensure!(
					smj.rigger == Rigger::Senior || smj.rigger == Rigger::Master,
					Error::<T>::NotQualified
				);

				// Ensure Reserve Exists & active
				let mut parachute_data = Reserves::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let nonce = ReservesRiggedNonce::<T>::get();

				// Create Rigging Metadata
				let new_details = RiggingDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					drogue: None,
					location,
					date,
				};

				// Insert Rigging Metadata to Storage
				ReservesRigged::<T>::set(nonce.clone(), Some(new_details));

				// Push Nonce to Parachute Metadata
				let mut new_rigs = parachute_data.rigs.clone().to_vec();
				new_rigs.push(nonce.clone());
				parachute_data.rigs = BoundedVec::try_from(new_rigs).unwrap();
				Reserves::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Metadata
				let mut new_smj_rigs = smj.reserves_rigged.clone().to_vec();
				new_smj_rigs.push(nonce.clone());
				smj.reserves_rigged = BoundedVec::try_from(new_smj_rigs).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				let nonce = nonce.saturating_add(1);
				ReservesRiggedNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRigged { who: rigger, parachute });
			},
		};

		Ok(())
	}

	// Repair a Parachute
	pub fn do_repair(
		rigger: T::AccountId,
		parachute: ParachuteId<T>,
		parachute_type: ParachuteType,
		date: BoundedVec<u8, T::MaxIdLen>,
		repair: BoundedVec<u8, T::MaxRepairLen>,
	) -> DispatchResult {
		// Ensure Rigger is Qualified
		let mut smj = Riggers::<T>::get(rigger.clone()).ok_or(Error::<T>::SmjDoesNotExist).unwrap();
		ensure!(
			smj.rigger == Rigger::Senior || smj.rigger == Rigger::Master,
			Error::<T>::NotQualified
		);

		match parachute_type {
			// Repair a Drogue
			ParachuteType::Drogue => {
				// Ensure Drogue Exists & active
				let mut parachute_data: ParachuteDetails<T> = Drogues::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let mut nonce = RepairNonce::<T>::get();

				// Create Repair Metadata
				let new_repair = RepairDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					date: date.clone(),
					repair: repair.clone(),
				};

				// Insert Repair into Storage
				Repair::<T>::set(nonce.clone(), Some(new_repair));

				// Push Nonce to Parachute Metadata
				let mut new_repair = parachute_data.repairs.clone().to_vec();
				new_repair.push(nonce.clone());
				parachute_data.repairs = BoundedVec::try_from(new_repair).unwrap();
				Drogues::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Loft Metadata
				let mut new_smj_repairs = smj.repairs.clone().to_vec();
				new_smj_repairs.push(nonce.clone());
				smj.repairs = BoundedVec::try_from(new_smj_repairs).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				nonce = nonce.saturating_add(1);
				RepairNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRepaired { who: rigger, parachute });
			},
			// Repair a Main Parachute
			ParachuteType::Main => {
				// Ensure Main Parachute Exists & Active
				let mut parachute_data: ParachuteDetails<T> = Mains::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let mut nonce = RepairNonce::<T>::get();

				// Create Repair Matadata
				let new_repair = RepairDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					date: date.clone(),
					repair: repair.clone(),
				};

				// Insert Repair into Storage
				Repair::<T>::set(nonce.clone(), Some(new_repair));

				// Push Nonce to Parachute Metadata
				let mut new_repair = parachute_data.repairs.clone().to_vec();
				new_repair.push(nonce.clone());
				parachute_data.repairs = BoundedVec::try_from(new_repair).unwrap();
				Mains::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Loft Metadata
				let mut new_smj_repairs = smj.repairs.clone().to_vec();
				new_smj_repairs.push(nonce.clone());
				smj.repairs = BoundedVec::try_from(new_smj_repairs).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				nonce = nonce.saturating_add(1);
				RepairNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRepaired { who: rigger, parachute });
			},
			// Repair a Reserve Parachute
			ParachuteType::Reserve => {
				// Ensure Reserve Exists & Active
				let mut parachute_data: ParachuteDetails<T> = Reserves::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let mut nonce = RepairNonce::<T>::get();

				// Create Repair Metadata
				let new_repair = RepairDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					date: date.clone(),
					repair: repair.clone(),
				};

				// Insert into Storage
				Repair::<T>::set(nonce.clone(), Some(new_repair));

				// Push Nonce to Parachute Metadata
				let mut new_repair = parachute_data.repairs.clone().to_vec();
				new_repair.push(nonce.clone());
				parachute_data.repairs = BoundedVec::try_from(new_repair).unwrap();
				Reserves::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Loft Metadata
				let mut new_smj_repairs = smj.repairs.clone().to_vec();
				new_smj_repairs.push(nonce.clone());
				smj.repairs = BoundedVec::try_from(new_smj_repairs).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				nonce = nonce.saturating_add(1);
				RepairNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRepaired { who: rigger, parachute });
			},
		}

		Ok(())
	}

	// Retire a Parachute
	pub fn do_retire(
		rigger: T::AccountId,
		parachute: ParachuteId<T>,
		parachute_type: ParachuteType,
		date: BoundedVec<u8, T::MaxIdLen>,
	) -> DispatchResult {
		// Ensure Rigger is Qualified
		let mut smj = Riggers::<T>::get(rigger.clone()).ok_or(Error::<T>::SmjDoesNotExist).unwrap();
		ensure!(
			smj.rigger == Rigger::Senior || smj.rigger == Rigger::Master,
			Error::<T>::NotQualified
		);

		match parachute_type {
			// Retire a Drogue
			ParachuteType::Drogue => {
				// Ensure Drogue Exists & Active
				let mut parachute_data: ParachuteDetails<T> = Drogues::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let mut nonce = RetireNonce::<T>::get();

				// Create Retire Metadata
				let new_retire = RetireDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					date: date.clone(),
				};

				// Insert Retire into Storage
				Retire::<T>::set(nonce.clone(), Some(new_retire));

				// Push Nonce to Parachute Metadata
				parachute_data.retired = Some(nonce.clone());
				parachute_data.active = false;
				Drogues::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Loft Metadata
				let mut new_smj_retires = smj.retires.clone().to_vec();
				new_smj_retires.push(nonce.clone());
				smj.retires = BoundedVec::try_from(new_smj_retires).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				nonce = nonce.saturating_add(1);
				RetireNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRetired { who: rigger, parachute });
			},
			// Retire a Main Parachute
			ParachuteType::Main => {
				// Ensure Main Parachute Exists & Active
				let mut parachute_data: ParachuteDetails<T> = Mains::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let mut nonce = RetireNonce::<T>::get();

				// Create Retire Metadata
				let new_retire = RetireDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					date: date.clone(),
				};

				// Insert Retire into Storage
				Retire::<T>::set(nonce.clone(), Some(new_retire));

				// Push Nonce to Parachute Metadata
				parachute_data.retired = Some(nonce.clone());
				parachute_data.active = false;
				Mains::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Loft Metadata
				let mut new_smj_retires = smj.retires.clone().to_vec();
				new_smj_retires.push(nonce.clone());
				smj.retires = BoundedVec::try_from(new_smj_retires).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				nonce = nonce.saturating_add(1);
				RetireNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRetired { who: rigger, parachute });
			},
			// Retire Reserve Parachute
			ParachuteType::Reserve => {
				// Ensure Reserve Parachute Exists & Active
				let mut parachute_data: ParachuteDetails<T> = Reserves::<T>::get(parachute.clone())
					.ok_or(Error::<T>::ParachuteDoesNotExist)
					.unwrap();
				ensure!(parachute_data.active == true, Error::<T>::ParachuteRetired);

				// Get Nonce
				let mut nonce = RetireNonce::<T>::get();

				// Create Retire Metadata
				let new_retire = RetireDetails {
					who: rigger.clone(),
					parachute: parachute.clone(),
					date: date.clone(),
				};

				// Insert Retire into Storage
				Retire::<T>::set(nonce.clone(), Some(new_retire));

				// Push Nonce to Parachute Metadata
				parachute_data.retired = Some(nonce.clone());
				parachute_data.active = false;
				Reserves::<T>::set(parachute.clone(), Some(parachute_data));

				// Push Nonce to SMJ Loft Metadata
				let mut new_smj_retires = smj.retires.clone().to_vec();
				new_smj_retires.push(nonce.clone());
				smj.retires = BoundedVec::try_from(new_smj_retires).unwrap();
				Riggers::<T>::set(rigger.clone(), Some(smj));

				// Update Nonce
				nonce = nonce.saturating_add(1);
				RetireNonce::<T>::set(nonce);

				Self::deposit_event(Event::<T>::ParachuteRetired { who: rigger, parachute });
			},
		};

		Ok(())
	}
}
