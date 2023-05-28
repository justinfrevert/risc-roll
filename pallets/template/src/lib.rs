#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod common;

#[frame_support::pallet]
pub mod pallet {
	use crate::common::TRANSFER_IMAGE_ID;
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;
	use risc0_zkvm::{serde::from_slice, sha::Digest, SegmentReceipt, SessionReceipt};
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
	}
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The seal was verified
		VerificationSuccess,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The seal could not be verified
		FailedVerification,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		BalanceOf<T>: From<u128>,
	{
		#[pallet::weight(1000000)]
		#[pallet::call_index(0)]
		pub fn submit_transfer_proofs(
			origin: OriginFor<T>,
			// All accounts that were changed, in order
			accounts: Vec<T::AccountId>,
			substrate_segment_receipts: Vec<(Vec<u32>, u32)>,
			// journal of (Vec<old balances>, Vec<new_balances>), both in order
			journal: Vec<u8>,
		) -> DispatchResult {
			// TODO: Look into whether there is a configuration where we don't need this extra
			// signature check due to the other verifications i.e. add the receipt verification in
			// the pallet validate unsigned portion
			ensure_signed(origin)?;
			let segments: Vec<SegmentReceipt> = substrate_segment_receipts
				.into_iter()
				.map(|(seal, index)| SegmentReceipt { seal, index })
				.collect();

			let receipt = SessionReceipt { segments, journal };

			receipt
				.verify(Digest::new(TRANSFER_IMAGE_ID))
				.map_err(|_| Error::<T>::FailedVerification)?;

			// sender original, sender final, recipient original, recipient final
			let (_, balances): (Vec<[u8; 16]>, Vec<[u8; 16]>) = from_slice(&receipt.journal).expect(
				"Journal output should deserialize into the same types (& order) that it was written",
			);

			accounts.into_iter().zip(balances.into_iter()).for_each(|(account, balance)| {
				let balance = u128::from_be_bytes(balance);
				// TODO: Check if there is a broader way to set new state
				T::Currency::make_free_balance_be(&account, balance.into());
			});

			Self::deposit_event(Event::<T>::VerificationSuccess);
			Ok(())
		}
	}
}
