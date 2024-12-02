use super::*;
use frame_support::{
    pallet_prelude::*,
    BoundedVec,
    traits::tokens::{ Fortitude::Force, Precision::BestEffort, Preservation::Preserve },
};
use scale_info::prelude::vec::Vec;

impl<T: Config> Pallet<T> {
    pub fn do_create_smj(
        new_smj: T::AccountId, 
        name: BoundedVec<u8, T::MaxLength>
    ) -> DispatchResult {
        ensure!(!Smokejumpers::<T>::contains_key(new_smj.clone()), Error::<T>::SmjAlreadyCreated);
        let smj_name = name.clone(); 
        let smokejumper = Smokejumper { name, rigger: Rigger::Uncertified };

        let gas_abstraction = T::NativeBalance::issue(420_000_000_000_000u128.saturated_into());
        T::NativeBalance::resolve(&new_smj.clone(), gas_abstraction).ok();
        Smokejumpers::<T>::insert(new_smj.clone(), smokejumper);

        Self::deposit_event(Event::<T>::SmjCreated { smj: smj_name });
        Ok(())
    }

    pub fn do_set_rigger(
        smj: T::AccountId,
        rigger: Rigger
    ) -> DispatchResult {
        let mut smj_data: Smokejumper<T> = Smokejumpers::<T>::get(smj.clone()).ok_or(Error::<T>::SmjDoesNotExist)?;
        let name = smj_data.name.clone();
        smj_data.rigger = rigger.clone();

        Smokejumpers::<T>::set(smj.clone(), Some(smj_data));
    
        Self::deposit_event(Event::<T>::RiggerChanged { smj: name, rigger });
        Ok(())
    }

    pub fn do_change_name(
        smj: T::AccountId,
        name: BoundedVec<u8, T::MaxLength>,
    ) -> DispatchResult {
        let mut smj_data: Smokejumper<T> = Smokejumpers::<T>::get(smj.clone()).ok_or(Error::<T>::SmjDoesNotExist)?;
        smj_data.name = name.clone();

        Smokejumpers::<T>::set(smj.clone(), Some(smj_data));

        Self::deposit_event(Event::<T>::NameChanged { smj: name });
        Ok(())
    }

    pub fn do_swap_address(
        smj: T::AccountId,
        new_id: T::AccountId,
    ) -> DispatchResult {
        ensure!(!Smokejumpers::<T>::contains_key(new_id.clone()), Error::<T>::AlreadySmj);
        ensure!(Smokejumpers::<T>::contains_key(smj.clone()), Error::<T>::SmjDoesNotExist);

        Smokejumpers::<T>::swap(smj.clone(), new_id.clone());
        let gas_abstraction = T::NativeBalance::issue(420_000_000_000_000u128.saturated_into());
        T::NativeBalance::resolve(&new_id.clone(), gas_abstraction).ok();

        let place_holder_name: Vec<u8> = "old_id_dont_use".as_bytes().to_vec().try_into().unwrap();
        let bounded_name: BoundedVec<u8, T::MaxLength> = BoundedVec::try_from(place_holder_name).unwrap();

        let place_holder = Smokejumper { 
            name: bounded_name,
            rigger: Rigger::Uncertified,
        };
        Smokejumpers::<T>::set(smj.clone(), Some(place_holder));
        let balance = T::NativeBalance::balance(&smj.clone());
        T::NativeBalance::burn_from(&smj.clone(), balance, Preserve, BestEffort, Force).ok();

        let name = Smokejumpers::<T>::get(new_id).ok_or(Error::<T>::SmjDoesNotExist).unwrap().name;
        Self::deposit_event(Event::<T>::IdSwap { smj: name });

        Ok(())
    }
}
