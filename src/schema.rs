use exonum_derive::*;
use crate::wallets::Wallet;
use exonum::crypto::{Hash, PublicKey};
use exonum::merkledb::{
    access::{Access, FromAccess},
    Fork, MapIndex, Snapshot,
};


#[derive(Debug, FromAccess)]
pub struct CurrencySchema<T: Access> {
    /// Correspondence of public keys of users to the account information.
    pub wallets: MapIndex<T::Base, PublicKey, Wallet>,
}

impl<T: Access> CurrencySchema<T> {
    pub fn new(access: T) -> Self {
        Self::from_root(access).unwrap()
    }
}

