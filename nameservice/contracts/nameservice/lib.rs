#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ensubdomainfactory {
    #[ink(storage)]
    pub struct EnsSubdomainFactory {
        owner: AccountId,
        locked: bool,
    }
    #[ink(event)]
    pub struct SubdomainCreated {
        #[ink(topic)] //-> indexed
        creator: AccountId,
        owner: AccountId,
        subdomain: Vec<u8>,
        domain: Vec<u8>,
    }
    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)] //-> indexed
        previous_owner: AccountId,
        new_owner: AccountId,
    }
    #[ink(event)]
    pub struct RegistryUpdated {
        #[ink(topic)] //-> indexed
        previous_registry: AccountId,
        new_registry: AccountId,
    }
    #[ink(event)]
    pub struct ResolverUpdated {
        #[ink(topic)] //-> indexed
        previous_resolver: AccountId,
        new_resolver: AccountId,
    }
    #[ink(event)]
    pub struct DomainTransfersLocked {
        #[ink(topic)] //-> indexed
        caller: AccountId,
        locked: bool,
    }

    impl EnsSubdomainFactory {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                locked: false,
            }
        }
        #[ink(message)]
        pub fn set_subdomain_owner(&mut self, subdomain: Vec<u8>, domain: Vec<u8>, owner: AccountId) {
            assert_eq!(self.env().caller(), self.owner);
            assert_eq!(self.locked, false);
            let subdomain_hash = self.namehash(subdomain, domain);
            //call setOwner on resolver
            self.env().emit_event(SubdomainCreated {
                creator: self.env().caller(),
                owner: owner,
                subdomain: subdomain,
                domain: domain,
            });
        }
        #[ink(message)]
        pub fn create_subdomain(&mut self, subdomain: Vec<u8>, domain: Vec<u8>) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can create subdomains");
            assert_eq!(self.locked, false, "Domain transfers are locked");
            let subdomain_hash = self.namehash(subdomain, domain);
            let subdomain_owner = self.getOwner(subdomain_hash);
            assert_eq!(subdomain_owner, AccountId::from([0x0; 32]), "Subdomain already exists");
            self.set_subdomain_owner(subdomain_hash, caller);
            self.env().emit_event(SubdomainCreated {
                creator: caller,
                owner: caller,
                subdomain: subdomain,
                domain: domain,
            });
        }
        #[ink(message)]
        pub fn transfer_subdomain(&mut self, subdomain: Vec<u8>, domain: Vec<u8>, new_owner: AccountId) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can transfer subdomains");
            assert_eq!(self.locked, false, "Domain transfers are locked");
            let subdomain_hash = self.namehash(subdomain, domain);
            let subdomain_owner = self.getOwner(subdomain_hash);
            assert_eq!(subdomain_owner, caller, "Only subdomain owner can transfer subdomain");
            self.set_subdomain_owner(subdomain_hash, new_owner);
            self.env().emit_event(SubdomainCreated {
                creator: caller,
                owner: new_owner,
                subdomain: subdomain,
                domain: domain,
            });
        }
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can transfer ownership");
            self.owner = new_owner;
            self.env().emit_event(OwnershipTransferred {
                previous_owner: caller,
                new_owner: new_owner,
            });
        }
        #[ink(message)]
        pub fn lock_domain_transfers(&mut self) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can lock domain transfers");
            self.locked = true;
            self.env().emit_event(DomainTransfersLocked {
                caller: caller,
                locked: self.locked
            });
        }
        #[ink(message)]
        pub fn transfer_contract_ownership(&mut self, new_owner: AccountId) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can transfer contract ownership");
            self.env().transfer(new_owner, self.env().balance());
        }
    }

}
