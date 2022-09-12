#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod governor {
    use ink_storage::{
        traits::*,
        Mapping,
    };
    use openbrush::contracts::traits::psp22::*;
    use scale::{
        Decode,
        Encode,
    };

    pub const ONE_MINUTE: u64 = 60 * 1000;

    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub enum VoteType {
        Against,
        For,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum GovernorError {
        AmountShouldNotBeZero,
        DurationError,
        ProposalNotFound,
        ProposalAlreadyExecuted,
        VotePeriodEnded,
        AlreadyVoted,
        VotePeriodNotEnded,
        QuorumNotReached,
        TransferError,
        ProposalNotAccepted,
    }

    #[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct Proposal {
        to: AccountId,
        amount: Balance,
        vote_start: Timestamp,
        vote_end: Timestamp,
        executed: bool,
    }

    #[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct ProposalVote {
        against_votes: u8,
        for_votes: u8,
    }

    pub type ProposalId = u32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Governor {
        proposal_votes: Mapping<ProposalId, ProposalVote>,
        proposals: Mapping<ProposalId, Proposal>,
        votes: Mapping<(ProposalId, AccountId), ()>,
        next_proposal_id: u32,
        quorum: u8,
        governance_token: AccountId,
    }

    impl Governor {
        #[ink(constructor, payable)]
        pub fn new(governance_token: AccountId, quorum: u8) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.quorum = quorum;
                instance.governance_token = governance_token;
            })
        }

        #[ink(message)]
        pub fn propose(
            &mut self,
            to: AccountId,
            amount: Balance,
            duration: u64,
        ) -> Result<(), GovernorError> {
            if amount == 0 {
                return Err(GovernorError::AmountShouldNotBeZero)
            }
            if duration == 0 || duration > 60 * ONE_MINUTE {
                return Err(GovernorError::DurationError)
            }

            let now = self.env().block_timestamp();
            let proposal = Proposal {
                to,
                amount,
                vote_start: now,
                vote_end: now + duration * ONE_MINUTE,
                executed: false,
            };

            let id = self.next_proposal_id();
            self.proposals.insert(id, &proposal);

            Ok(())
        }

        #[ink(message)]
        pub fn vote(
            &mut self,
            proposal_id: ProposalId,
            vote: VoteType,
        ) -> Result<(), GovernorError> {
            let caller = self.env().caller();
            let proposal = self
                .proposals
                .get(&proposal_id)
                .ok_or(GovernorError::ProposalNotFound)?;
            if proposal.executed {
                return Err(GovernorError::ProposalAlreadyExecuted)
            }
            let now = self.env().block_timestamp();
            if now > proposal.vote_end {
                return Err(GovernorError::VotePeriodEnded)
            }
            if self.votes.get(&(proposal_id, caller)).is_some() {
                return Err(GovernorError::AlreadyVoted)
            }

            self.votes.insert(&(proposal_id, caller), &());

            let weight = self.account_weight(caller);
            let mut proposal_vote = self.proposal_votes.get(proposal_id).unwrap_or_default();
            match vote {
                VoteType::Against => {
                    proposal_vote.against_votes += weight;
                }
                VoteType::For => {
                    proposal_vote.for_votes += weight;
                }
            }

            self.proposal_votes.insert(&proposal_id, &proposal_vote);
            Ok(())
        }

        #[ink(message)]
        pub fn execute(&mut self, proposal_id: ProposalId) -> Result<(), GovernorError> {
            let mut proposal = self
                .proposals
                .get(&proposal_id)
                .ok_or(GovernorError::ProposalNotFound)?;
            if proposal.executed {
                return Err(GovernorError::ProposalAlreadyExecuted)
            }
            let proposal_vote = self.proposal_votes.get(proposal_id).unwrap_or_default();
            if proposal_vote.for_votes + proposal_vote.against_votes < self.quorum {
                return Err(GovernorError::QuorumNotReached)
            }
            if proposal_vote.against_votes >= proposal_vote.for_votes {
                return Err(GovernorError::ProposalNotAccepted)
            }

            proposal.executed = true;
            self.env()
                .transfer(proposal.to, proposal.amount)
                .map_err(|_| GovernorError::TransferError)?;

            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal_vote(&self, proposal_id: ProposalId) -> Option<ProposalVote> {
            self.proposal_votes.get(proposal_id)
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
            self.proposals.get(proposal_id)
        }

        #[ink(message)]
        pub fn has_voted(&self, proposal_id: u32, account_id: AccountId) -> bool {
            self.votes.get(&(proposal_id, account_id)).is_some()
        }

        fn account_weight(&self, caller: AccountId) -> u8 {
            let balance = PSP22Ref::balance_of(&self.governance_token, caller);
            let total_supply = PSP22Ref::total_supply(&self.governance_token);
            (balance * 100 / total_supply) as u8
        }

        fn next_proposal_id(&mut self) -> ProposalId {
            let id = self.next_proposal_id;
            self.next_proposal_id += 1;
            id
        }
    }
}
