# Decentralized Voting Pallet

## Overview

This Substrate pallet implements a decentralized voting system. It allows users to create proposals, vote on them, and finalize their results. The pallet utilizes the Substrate runtime and timestamp pallet to manage proposal creation, voting, and proposal finalization based on a timestamped voting period.

## Features

- **Create Proposals**: Users can create new proposals by submitting a description and a voting end timestamp.
- **Vote on Proposals**: Users can cast votes on active proposals. Each user can vote only once per proposal.
- **Finalize Proposals**: Once the voting period ends, proposals are finalized based on the majority of votes.

## Setup Instructions

1. Clone the repository

```bash
git clone https://github.com/milosdjurica/mvp-substrate-voting
```

2. Install dependencies

```bash
cargo build --release
```

## Run pallet in demo mode

1. Start the node in development mode

```bash
./target/release/node-template --dev
```

2. Run @polkadot/apps. Click on the following link -> https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer

- It is a "Portal" into the Polkadot and Substrate networks. Provides a view and interaction layer from a browser. Playground in browser where you can interact with your pallet.
- GitHub Project link -> https://github.com/polkadot-js/apps .

3. Under `Developer` tab you have `Chain state` and `Extrinsics` options. Under `Chain state` tab you can read from read functions. Under `Extrinsics` tab you can `create_proposal()` , `vote()` and `finalize_proposal()`.

## Pallet functionality

### `create_proposal()`

#### Parameters

- `origin` - Default parameter representing user that is calling function
- `description` - Description of the proposal - Description can NOT be longer than 1024 bytes. That means the description can hold a maximum of 1,024 characters if they are all ASCII characters (each taking 1 byte).
- `end_timestamp` - Represents the moment in which the proposal can be finalized. Must at least `86_400_000` (one day) longer than the `current_timestamp`

#### Errors

- Can throw the following errors :
  - `Overflow` - If Proposal ID is too big
  - `EndTimeStampTooSoon` - If `end_timestamp` is shorter than 1 day from current moment in time
  - `DescriptionIsTooLong` - If `description` is longer than 1024 bytes

#### State changes

- Inserts new proposal in `ActiveProposals` - Mapping of all active proposals. Maps index of proposal to Proposal.
- Increments `ProposalCounter` - index of proposal

#### Event emitting

- Emits `ProposalCreated` event

  ```rust
  ProposalCreated {
  		proposal_id: u32,
  		creator: T::AccountId,
  		description: Vec<u8>,
  		end_timestamp: T::Moment,
  	},

  ```

### `vote()`

#### Parameters

- `origin` - The user who is casting the vote.
- `proposal_id` - The ID of the proposal on which the vote is being cast.
- `vote_is_yes` - A boolean value representing the user's vote. `true` indicates a vote in favor of the proposal, and `false` indicates a vote against it.

#### Errors

- Can throw the following errors:
  - `ProposalDoesNotExist` - Thrown if the `proposal_id` does not correspond to an existing active proposal.
  - `ProposalIsNotActive` - Thrown if the current timestamp is beyond the proposal's `end_timestamp`, preventing from users to vote on proposal after it should be finalized.
  - `UserAlreadyVoted` - Thrown if the user has already voted on the proposal.

#### State changes

- Appends the user's vote to the `ProposalToVotes` mapping, which stores the list of votes for each proposal.
- Updates the `UserHasVotedOnProposal` mapping to record that the user has voted on this specific proposal.

#### Event emitting

- Emits the `UserVoted` event:

  ```rust
  UserVoted {
      proposal_id: u32,
      voter: T::AccountId,
      vote_is_yes: bool,
  },
  ```
