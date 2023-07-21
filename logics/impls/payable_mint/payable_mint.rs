// Copyright (c) 2022 Astar Network
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use ink::prelude::string::{
    String as PreludeString,
    ToString,
};

use crate::impls::payable_mint::types::{
    Data,
    Shiden34Error,
};
use openbrush::{
    // contracts::{
    //     ownable::*,
    //     psp34::extensions::{
    //         enumerable::*,
    //         metadata::*,
    //     },
    //     reentrancy_guard::*,
    // },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
    },
};

use openbrush::contracts::{
    ownable,
    ownable::only_owner,
    psp34,
    psp34::{
        extensions::{
            metadata,
            metadata::{
                Id,
                PSP34MetadataImpl,
            },
        },
        PSP34Error,
        PSP34Impl,
    },
    reentrancy_guard,
    reentrancy_guard::non_reentrant,
};

#[openbrush::trait_definition]
pub trait PayableMintImpl:
    Storage<Data>
    + Storage<psp34::Data>
    + Storage<reentrancy_guard::Data>
    + Storage<ownable::Data>
    + Storage<metadata::Data>
    + PSP34Impl
    + PSP34MetadataImpl
    + psp34::extensions::metadata::Internal
    + Internal
{
    /// Mint one or more tokens
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
        self.check_amount(mint_amount)?;
        self.check_value(Self::env().transferred_value(), mint_amount)?;

        let next_to_mint = self.data::<Data>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            self._mint_to(to, Id::U64(mint_id))?;
            self.data::<Data>().last_token_id += 1;
            self._emit_transfer_event(None, Some(to), Id::U64(mint_id));
        }

        Ok(())
    }

    /// Mint next available token for the caller
    #[ink(message, payable)]
    fn mint_next(&mut self) -> Result<(), PSP34Error> {
        self.check_value(Self::env().transferred_value(), 1)?;
        let caller = Self::env().caller();
        let token_id = self
            .data::<Data>()
            .last_token_id
            .checked_add(1)
            .ok_or(PSP34Error::Custom(Shiden34Error::CollectionIsFull.as_str()))?;
        self._mint_to(caller, Id::U64(token_id))?;
        self.data::<Data>().last_token_id += 1;

        self._emit_transfer_event(None, Some(caller), Id::U64(token_id));

        Ok(())
    }

    /// Set new value for the baseUri
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
        let id = PSP34Impl::collection_id(self);
        metadata::Internal::_set_attribute(self, id, String::from("baseUri"), uri);

        Ok(())
    }

    /// Withdraws funds to contract owner
    #[ink(message)]
    #[modifiers(only_owner)]
    fn withdraw(&mut self) -> Result<(), PSP34Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        let owner = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
        Self::env()
            .transfer(owner, current_balance)
            .map_err(|_| PSP34Error::Custom(Shiden34Error::WithdrawalFailed.as_str()))?;
        Ok(())
    }

    /// Set max number of tokens which could be minted per call
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_max_mint_amount(&mut self, max_amount: u64) -> Result<(), PSP34Error> {
        self.data::<Data>().max_amount = max_amount;

        Ok(())
    }

    /// Get URI from token ID
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<PreludeString, PSP34Error> {
        self.token_exists(Id::U64(token_id))?;
        let base_uri = PSP34MetadataImpl::get_attribute(
            self,
            PSP34Impl::collection_id(self),
            String::from("baseUri"),
        );
        let token_uri = base_uri.unwrap() + &token_id.to_string() + &PreludeString::from(".json");
        Ok(token_uri)
    }

    /// Get max supply of tokens
    #[ink(message)]
    fn max_supply(&self) -> u64 {
        self.data::<Data>().max_supply
    }

    /// Get token price
    #[ink(message)]
    fn price(&self) -> Balance {
        self.data::<Data>().price_per_mint
    }

    /// Get max number of tokens which could be minted per call
    #[ink(message)]
    fn get_max_mint_amount(&mut self) -> u64 {
        self.data::<Data>().max_amount
    }
}

/// Helper trait for PayableMint
pub trait Internal: Storage<Data> + psp34::Internal {
    /// Check if the transferred mint values is as expected
    fn check_value(&self, transferred_value: u128, mint_amount: u64) -> Result<(), PSP34Error> {
        if let Some(value) = (mint_amount as u128).checked_mul(self.data::<Data>().price_per_mint) {
            if transferred_value == value {
                return Ok(())
            }
        }
        Err(PSP34Error::Custom(Shiden34Error::BadMintValue.as_str()))
    }

    /// Check amount of tokens to be minted
    fn check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom(
                Shiden34Error::CannotMintZeroTokens.as_str(),
            ))
        }
        if mint_amount > self.data::<Data>().max_amount {
            return Err(PSP34Error::Custom(
                Shiden34Error::TooManyTokensToMint.as_str(),
            ))
        }
        if let Some(amount) = self.data::<Data>().last_token_id.checked_add(mint_amount) {
            if amount <= self.data::<Data>().max_supply {
                return Ok(())
            }
        }

        Err(PSP34Error::Custom(Shiden34Error::CollectionIsFull.as_str()))
    }

    /// Check if token is minted
    fn token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self._owner_of(&id).ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}
