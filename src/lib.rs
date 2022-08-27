pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use exonum::crypto::{Hash, PublicKey};
use exonum::merkledb::{
    access::{Access, FromAccess},
    Fork, MapIndex, Snapshot,
};
use exonum::runtime::{ExecutionContext, ExecutionError};
use exonum_derive::*;
use exonum_proto::ProtobufConvert;
use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};
use exonum_rust_runtime::Service;

use serde_derive::{Deserialize, Serialize};

// Starting balance of a newly created wallet
const INIT_BALANCE: u64 = 100;
pub mod proto;
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Wallet")]
pub struct Wallet {
    pub pub_key: PublicKey,
    pub name: String,
    pub balance: u64,
}
impl Wallet {
    pub fn new(&pub_key: &PublicKey, name: &str, balance: u64) -> Self {
        Self {
            pub_key,
            name: name.to_owned(),
            balance,
        }
    }

    pub fn increase(self, amount: u64) -> Self {
        let balance = self.balance + amount;
        Self::new(&self.pub_key, &self.name, balance)
    }

    pub fn decrease(self, amount: u64) -> Self {
        debug_assert!(self.balance >= amount);
        let balance = self.balance - amount;
        Self::new(&self.pub_key, &self.name, balance)
    }
}


