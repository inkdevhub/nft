#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ensubdomainfactory {
    #[ink(storage)]
    pub struct EnsSubdomainFactory {
        owner: AccountId;
        locked: bool;
    }
    #[ink(event)]
    pub struct SubdomainCreated {
        #[ink(topic)] //-> indexed
        creator: AccountId,
        owner: AccountId,
        subdomain: vec[u8],
        domain: vec[u8],
    }
    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)] //-> indexed
        previousOwner: AccountId,
        newOwner: AccountId,
    }
    #[ink(event)]
    pub struct RegistryUpdated {
        #[ink(topic)] //-> indexed
        previousRegistry: AccountId,
        newRegistry: AccountId,
    }
    #[ink(event)]
    pub struct ResolverUpdated {
        #[ink(topic)] //-> indexed
        previousResolver: AccountId,
        newResolver: AccountId,
    }
    #[ink(event)]
    pub struct DomainTransfersLocked {
        #[ink(topic)] //-> indexed
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
        pub fn createSubdomain(&mut self, subdomain: vec[u8], domain: vec[u8]) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can create subdomains");
            assert_eq!(self.locked, false, "Domain transfers are locked");
            let subdomainHash = self.namehash(subdomain, domain);
            let subdomainOwner = self.getOwner(subdomainHash);
            assert_eq!(subdomainOwner, AccountId::from([0x0; 32]), "Subdomain already exists");
            self.setSubdomainOwner(subdomainHash, caller);
            self.env().emit_event(SubdomainCreated {
                creator: caller,
                owner: caller,
                subdomain: subdomain,
                domain: domain,
            });
        }
        #[ink(message)]
        pub fn transferSubdomain(&mut self, subdomain: vec[u8], domain: vec[u8], newOwner: AccountId) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can transfer subdomains");
            assert_eq!(self.locked, false, "Domain transfers are locked");
            let subdomainHash = self.namehash(subdomain, domain);
            let subdomainOwner = self.getOwner(subdomainHash);
            assert_eq!(subdomainOwner, caller, "Only subdomain owner can transfer subdomain");
            self.setSubdomainOwner(subdomainHash, newOwner);
            self.env().emit_event(SubdomainCreated {
                creator: caller,
                owner: newOwner,
                subdomain: subdomain,
                domain: domain,
            });
        }
        #[ink(message)]
        pub fn transferOwnership(&mut self, newOwner: AccountId) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can transfer ownership");
            self.owner = newOwner;
            self.env().emit_event(OwnershipTransferred {
                previousOwner: caller,
                newOwner: newOwner,
            });
        }
        #[ink(message)]
        pub fn lockDomainTransfers(&mut self) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can lock domain transfers");
            self.locked = true;
            self.env().emit_event(DomainTransfersLocked {});
        }
        #[ink(message)]
        pub fn transferContractOwnership(&mut self, newOwner: AccountId) {
            let caller = Self::env().caller();
            assert_eq!(self.owner, caller, "Only owner can transfer contract ownership");
            self.env().transfer(newOwner, self.env().balance());
        }
    }

}
