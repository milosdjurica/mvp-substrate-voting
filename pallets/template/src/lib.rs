// All pallets must be configured for `no_std`.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_timestamp::pallet::Config as TimestampConfig;
	use scale_info::prelude::vec::Vec;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + TimestampConfig {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Time: From<<Self as TimestampConfig>::Moment>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProposalCreated {
			proposal_id: u32,
			creator: T::AccountId,
			description: Vec<u8>,
			end_timestamp: T::Moment,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		EndTimeStampTooSoon,
		DescriptionIsTooLong,
	}

	#[pallet::storage]
	#[pallet::getter(fn proposal_counter)]
	pub(super) type ProposalCounter<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
