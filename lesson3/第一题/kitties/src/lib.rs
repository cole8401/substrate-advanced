#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use sp_io::hashing::blake2_128;
	use frame_support::traits::{Randomness,Currency,ExistenceRequirement};
	use frame_support::PalletId;
	use sp_runtime::traits::AccountIdConversion;
	
	pub type KittyId = u32;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub[u8; 16]);

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Randomness:Randomness<Self::Hash, Self::BlockNumber>;
		type Currency :Currency<Self::AccountId>;
		
		#[pallet::constant]
		type KittyPrice: Get<BalanceOf<Self>>;
		type PalletId:Get<PalletId>;
	}


	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T> = StorageValue<_, KittyId,ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId,Kitty>;


	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T:Config> = StorageMap<_, Blake2_128Concat, KittyId,T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T:Config> = StorageMap<_, Blake2_128Concat, KittyId,(KittyId,KittyId),OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T:Config> = StorageMap<_, Blake2_128Concat, KittyId,()>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyCreated{who: T::AccountId,kitty_id: KittyId, kitty:Kitty },
		KittyBred{who:T::AccountId, kitty_id:KittyId, kitty: Kitty},
		KittyTransferred{who:T::AccountId, recipient:T::AccountId, kitty_id: KittyId},
		KittyOnSale{who:T::AccountId, kitty_id: KittyId},
		KittyBought{who:T::AccountId, kitty_id: KittyId},

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		InvalidKittyId,
		NotOwner,
		SamedKittyId,
		AlreadyOnSale,
		NotOnSale,
		NoOwner,
		AlreadyOwned,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id()?;
			let kitty = Kitty(Self::random_value(&who));
			
			let price = T::KittyPrice::get();
			//T::Currency::reserve(&who, price)?;
			T::Currency::transfer(&who, &Self::get_account_id(), price,ExistenceRequirement::KeepAlive)?;

			Kitties::<T>::insert(kitty_id,&kitty);
			KittyOwner::<T>::insert(kitty_id,&who);


			Self::deposit_event(Event::KittyCreated{who,kitty_id,kitty });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyId,
			kitty_id_2: KittyId
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SamedKittyId);

			ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);

			let kitty_id = Self::get_next_id()?;
			let kitty_1 = Self::kitties(kitty_id_1).expect("We checked it exists");
			let kitty_2 = Self::kitties(kitty_id_2).expect("We checked it exists");

			let selector = Self::random_value(&who);
			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]) ;
			}
			let kitty = Kitty(data);

			let price = T::KittyPrice::get();
			//T::Currency::reserve(&who,price)?;
			T::Currency::transfer(&who,&Self::get_account_id(),price,ExistenceRequirement::KeepAlive)?;

			Kitties::<T>::insert(kitty_id,&kitty);
			KittyOwner::<T>::insert(kitty_id,&who);
			KittyParents::<T>::insert(kitty_id,(kitty_id_1,kitty_id_2));
			NextKittyId::<T>::set(kitty_id + 1);

			Self::deposit_event(Event::KittyBred{who,kitty_id,kitty});
			Ok(())
		}	

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 )]	
		pub fn transfer(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			kitty_id: KittyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(KittyOwner::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

			let owner = Self::kitty_owner(kitty_id).expect("We checked it exists");
			ensure!(owner == who, Error::<T>::NotOwner);
			KittyOwner::<T>::insert(kitty_id, &recipient);

			Self::deposit_event(Event::KittyTransferred {who, recipient, kitty_id });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 )]	
		pub fn sale(
			origin: OriginFor<T>,
			kitty_id: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);
			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::AlreadyOnSale);

			<KittyOnSale<T>>::insert(kitty_id, ());
			Self::deposit_event(Event::KittyOnSale{who, kitty_id});
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 )]	
		pub fn buy(
				origin: OriginFor<T>,	
				kitty_id: u32,
		)
			-> DispatchResult {
				let who = ensure_signed(origin)?;

				Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
				let owner = Self::kitty_owner(kitty_id).ok_or::<DispatchError>(Error::<T>::NotOwner.into())?;
				ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::NotOnSale);

				let price = T::KittyPrice::get();
				//T::Currency::reserve(&who,  price)?;
				//T::Currency::unreserve(&owner, price);

				<KittyOwner<T>>::insert(kitty_id, &who);
				<KittyOnSale<T>>::remove(kitty_id);

				Self::deposit_event(Event::KittyBought{ who, kitty_id});
				Ok(())
		}
	}

	impl<T:Config> Pallet<T> {
		fn get_next_id() -> Result<KittyId,DispatchError>{
			NextKittyId::<T>::try_mutate(|next_id|-> Result<KittyId,DispatchError>{
				let current_id = *next_id;
				*next_id = next_id.checked_add(1).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
				Ok(current_id)
			})
		}

		fn random_value(sender:&T::AccountId) -> [u8;16]{
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),	
			);
			payload.using_encoded(blake2_128)
		}
		fn get_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}