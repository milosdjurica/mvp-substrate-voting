<a name="readme-top"></a>

<!-- TABLE OF CONTENTS -->

## Table of Contents

  <ol>
    <li>
      <a href="#overview">Overview</a>
    </li>
    <li>
      <a href="#features">Features</a>
    </li>
    <li>
      <a href="#setup-instructions">Setup Instructions</a>
    </li>
    <li>
      <a href="#run-pallet-in-demo-mode">Run Pallet in Demo Mode</a>
    </li>
    <li>
      <a href="#pallet-functionality">Pallet Functionality</a>
      <ul>
        <li><a href="#create_proposal">create_proposal()</a></li>
        <li><a href="#vote">vote()</a></li>
        <li><a href="#finalize_proposal">finalize_proposal()</a></li>
        <li><a href="#count_votes">count_votes()</a></li>
      </ul>
    </li>
    <li>
      <a href="#potential-improvements">Potential improvements</a>
    </li>
  </ol>

<p align="right">(<a href="#readme-top">back to top</a>)</p>

# Decentralized Voting Pallet

## Overview

This Substrate pallet implements a decentralized voting system. It allows users to create proposals, vote on them, and finalize their results. The pallet utilizes the Substrate runtime and timestamp pallet to manage proposal creation, voting, and proposal finalization based on a timestamped voting period.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Features

- **Create Proposals**: Users can create new proposals by submitting a description and a voting end timestamp.
- **Vote on Proposals**: Users can cast votes on active proposals. Each user can vote only once per proposal.
- **Finalize Proposals**: Once the voting period ends, proposals are finalized based on the majority of votes.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Setup Instructions

1. Clone the repository

```bash
git clone https://github.com/milosdjurica/mvp-substrate-voting
```

2. Install dependencies

```bash
cargo build --release
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Run pallet in demo mode

1. Start the node in development mode

```bash
./target/release/node-template --dev
```

2. Run @polkadot/apps. Click on the following link -> https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer

- It is a "Portal" into the Polkadot and Substrate networks. Provides a view and interaction layer from a browser. Playground in browser where you can interact with your pallet.
- GitHub Project link -> https://github.com/polkadot-js/apps .

3. Under `Developer` tab you have `Chain state` and `Extrinsics` options. Under `Chain state` tab you can read from read functions. Under `Extrinsics` tab you can `create_proposal()` , `vote()` and `finalize_proposal()`.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Pallet functionality

### `create_proposal()`

#### Parameters

- `origin` - Default parameter representing user that is calling function
- `description` - Description of the proposal - Description can NOT be longer than 1024 bytes. That means the description can hold a maximum of 1,024 characters if they are all ASCII characters (each taking 1 byte).
- `proposal_duration_in_milliseconds` - Duration of proposal. Must be at least `86_400_000` (one day).

#### Errors

- Can throw the following errors :
  - `Overflow` - If Proposal ID is too big
  - `ProposalDurationTooShort` - If `proposal_duration_in_milliseconds` is shorter than 1 day (86_400_000)
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

<p align="right">(<a href="#readme-top">back to top</a>)</p>

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

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### `finalize_proposal()`

#### Parameters

- `origin` - The user who is attempting to finalize the proposal.
- `proposal_id` - The ID of the proposal that is being finalized.

#### Errors

- Can throw the following errors:
  - `ProposalDoesNotExist` - Thrown if the `proposal_id` does not exist in active proposals.
  - `TooEarlyToFinalize` - Thrown if the current timestamp is not yet beyond the proposal's `end_timestamp`.

#### State changes

- Removes the proposal from `ActiveProposals`.
- Adds proposal into `FinishedProposals`

#### Event emitting

- Emits the `ProposalFinalized` event:

  ```rust
  ProposalFinalized {
      proposal_id: u32,
      proposal_status: ProposalStatus,
      total_votes: u32,
      yes_votes: u32,
  },

  pub enum ProposalStatus{
    APPROVED,
    REJECTED,
  }
  ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### `count_votes()`

#### Parameters

- `proposal_id` - The ID of the proposal for which the vote counts are being retrieved.

#### Usage

- This function is used internally by the pallet in the `finalize_proposal()` function, to determine the outcome of a proposal based on the votes.

#### Return Values

- Returns a tuple `(total_votes, yes_votes)`:
  - `total_votes` - The total number of votes cast on the proposal.
  - `yes_votes` - The number of votes that were in favor of the proposal ( `YES` votes).

```rust
fn count_votes(proposal_id: u32) -> (u32, u32) {
    let votes = Self::proposal_to_votes(proposal_id).unwrap_or_default();
    let total_votes = votes.len() as u32;
    let yes_votes = votes.iter().filter(|v| v.vote_is_yes).count() as u32;

    (total_votes, yes_votes)
}
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Potential Improvements

- Automating finalizing proposals with `pallet-scheduler`.

<p align="right">(<a href="#readme-top">back to top</a>)</p>
