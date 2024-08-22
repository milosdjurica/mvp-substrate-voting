
// All pallets must be configured for `no_std`.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
 use frame_support::pallet_prelude::*;
 use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;






 #[pallet::pallet]
 
	#[pallet::without_storage_info]
 pub struct Pallet<T>(_);

#[pallet::config]
	pub trait Config: frame_system::Config + TimestampConfig {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Time: From<<Self as TimestampConfig>::Moment>;
	}




 #[pallet::event]   // <-- Step 3. code block will replace this.
 #[pallet::error]   // <-- Step 4. code block will replace this.
 #[pallet::storage] // <-- Step 5. code block will replace this.
 #[pallet::call]    // <-- Step 6. code block will replace this.
}

