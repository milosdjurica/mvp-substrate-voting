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

	#[derive(Clone, Encode, Decode, Default, TypeInfo)]
	pub struct Proposal<AccountId, Moment> {
		pub creator: AccountId,
		pub description: Vec<u8>,
		pub end_timestamp: Moment,
	}

	#[derive(Clone, Encode, Decode, Default, TypeInfo)]
	pub struct Vote<AccountId> {
		pub voter: AccountId,
		pub vote_is_yes: bool,
	}

	#[derive(Clone, Encode, Decode, Default, TypeInfo)]
	pub struct FinishedProposal<AccountId, Moment> {
		pub proposal: Proposal<AccountId, Moment>,
		pub is_approved: bool,
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
		UserVoted {
			proposal_id: u32,
			voter: T::AccountId,
			vote_is_yes: bool,
		},
		ProposalFinalized {
			proposal_id: u32,
			is_approved: bool,
			total_votes: u32,
			yes_votes: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		EndTimeStampTooSoon,
		DescriptionIsTooLong,
		Overflow,
		ProposalDoesNotExist,
		ProposalIsNotActive,
		UserAlreadyVoted,
		TooEarlyToFinalize,
	}

	#[pallet::storage]
	#[pallet::getter(fn proposal_counter)]
	pub(super) type ProposalCounter<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn active_proposals)]
	pub(super) type ActiveProposals<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Proposal<T::AccountId, T::Moment>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn proposal_to_votes)]
	pub(super) type ProposalToVotes<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,                     // Proposal ID
		Vec<Vote<T::AccountId>>, // List of votes
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn user_has_voted_on_proposal)]
	pub(super) type UserHasVotedOnProposal<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId, u32), // AccountId, Proposal ID
		bool,                // Did vote or did not vote
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn finished_proposals)]
	pub(super) type FinishedProposals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		FinishedProposal<T::AccountId, T::Moment>,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(Weight::default())]
		#[pallet::call_index(0)]
		pub fn create_proposal(
			origin: OriginFor<T>,
			description: Vec<u8>,
			end_timestamp: T::Moment,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let old_id = Self::proposal_counter();
			let new_id = old_id.checked_add(1).ok_or(Error::<T>::Overflow)?;

			// ! Check if end_timestamp  > current + min_diff
			let current_timestamp = <pallet_timestamp::Pallet<T>>::now();
			let min_difference: T::Moment =
				86_400_000u64.try_into().map_err(|_| Error::<T>::Overflow)?; // 1 day in milliseconds
			ensure!(
				end_timestamp > current_timestamp + min_difference,
				Error::<T>::EndTimeStampTooSoon
			);

			// ! Check if description too long
			let max_description_length = 1024u32;
			ensure!(
				description.len() as u32 <= max_description_length,
				Error::<T>::DescriptionIsTooLong
			);

			let new_proposal = Proposal {
				creator: sender.clone(),
				description: description.clone(),
				end_timestamp,
			};

			<ActiveProposals<T>>::insert(new_id, new_proposal);
			<ProposalCounter<T>>::put(new_id);

			Self::deposit_event(Event::ProposalCreated {
				proposal_id: new_id,
				creator: sender,
				description: description.clone(),
				end_timestamp,
			});

			Ok(())
		}

		#[pallet::weight(Weight::default())]
		#[pallet::call_index(1)]
		pub fn vote(origin: OriginFor<T>, proposal_id: u32, vote_is_yes: bool) -> DispatchResult {
			let voter = ensure_signed(origin)?;

			// ! Check if proposal exist
			let proposal =
				Self::active_proposals(proposal_id).ok_or(Error::<T>::ProposalDoesNotExist)?;
			let current_timestamp = <pallet_timestamp::Pallet<T>>::now();
			// ! Check if proposal is active
			ensure!(current_timestamp <= proposal.end_timestamp, Error::<T>::ProposalIsNotActive);

			// ! Check if user already voted
			ensure!(
				!Self::user_has_voted_on_proposal((voter.clone(), proposal_id)),
				Error::<T>::UserAlreadyVoted
			);

			// ! Vote
			let vote = Vote { voter: voter.clone(), vote_is_yes };
			<ProposalToVotes<T>>::append(proposal_id, vote);
			<UserHasVotedOnProposal<T>>::insert((voter.clone(), proposal_id), true);

			// ! Emit
			Self::deposit_event(Event::UserVoted {
				proposal_id,
				voter: voter.clone(),
				vote_is_yes,
			});

			Ok(())
		}

		#[pallet::weight(Weight::default())]
		#[pallet::call_index(2)]
		pub fn finalize_proposal(origin: OriginFor<T>, proposal_id: u32) -> DispatchResult {
			let _caller = ensure_signed(origin)?;

			let proposal = Self::active_proposals(proposal_id)
				.ok_or(Error::<T>::ProposalDoesNotExist)
				.unwrap();

			let current_timestamp = <pallet_timestamp::Pallet<T>>::now();
			ensure!(current_timestamp > proposal.end_timestamp, Error::<T>::TooEarlyToFinalize,);

			let (total_votes, yes_votes) = Self::get_vote_counts(proposal_id);

			let finished_proposal =
				FinishedProposal { proposal, is_approved: yes_votes * 2 > total_votes };

			<FinishedProposals<T>>::insert(proposal_id, finished_proposal.clone());
			<ActiveProposals<T>>::remove(proposal_id);

			// ! Emit number of YES vs number of NO
			Self::deposit_event(Event::ProposalFinalized {
				proposal_id,
				is_approved: finished_proposal.is_approved,
				total_votes,
				yes_votes,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_vote_counts(proposal_id: u32) -> (u32, u32) {
			let votes = Self::proposal_to_votes(proposal_id).unwrap_or_default();
			let total_votes = votes.len() as u32;
			let yes_votes = votes.iter().filter(|v| v.vote_is_yes).count() as u32;

			(total_votes, yes_votes)
		}
	}
}
