#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::{Currency, Randomness, tokens::Balance}};
    use frame_system::{pallet_prelude::*};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;
		type CollectionRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

		#[pallet::constant]
		type MaximumOwned: Get<u32>;
    }

	type BalanceOf<T> =	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub enum Color {Red, Yellow,Blue,Green}
	
	// #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Collectible<T:Config>{
		pub unique_id:[u8;16],
		pub price: Option<BalanceOf<T>>,
		pub color: Color,
		pub owner: T::AccountId,
	}
}