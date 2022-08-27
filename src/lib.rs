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

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert, BinaryValue)]
#[protobuf_convert(source = "proto::TxCreateWallet")]
pub struct TxCreateWallet {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert, BinaryValue)]
#[protobuf_convert(source = "proto::TxTransfer")]
pub struct TxTransfer {
    pub to: PublicKey,
    pub amount: u64,
    pub seed: u64,
}

/// Cryptocurrency service transactions.
#[exonum_interface]
pub trait CryptocurrencyInterface<Ctx> {
    /// Output of the methods in this interface.
    type Output;

    /// Creates wallet with the given `name`.
    #[interface_method(id = 0)]
    fn create_wallet(&self, ctx: Ctx, arg: TxCreateWallet) -> Self::Output;
    /// Transfers `amount` of the currency from one wallet to another.
    #[interface_method(id = 1)]
    fn transfer(&self, ctx: Ctx, arg: TxTransfer) -> Self::Output;
}

/// Error codes emitted by `TxCreateWallet` and/or `TxTransfer`
/// transactions during execution.
#[derive(Debug, ExecutionFail)]
pub enum Error {
    /// Wallet already exists.
    WalletAlreadyExists = 0,
    /// Sender doesn't exist.
    SenderNotFound = 1,
    /// Receiver doesn't exist.
    ReceiverNotFound = 2,
    /// Insufficient currency amount.
    InsufficientCurrencyAmount = 3,
    /// Sender same as receiver.
    SenderSameAsReceiver = 4,
}

/// Cryptocurrency service implementation.
#[derive(Debug, ServiceFactory, ServiceDispatcher)]
#[service_dispatcher(implements("CryptocurrencyInterface"))]
#[service_factory(proto_sources = "crate::proto")]
pub struct CryptocurrencyService;

impl Service for CryptocurrencyService {}

impl CryptocurrencyInterface<ExecutionContext<'_>> for CryptocurrencyService {
    type Output = Result<(), ExecutionError>;

    fn create_wallet(
        &self,
        context: ExecutionContext<'_>,
        arg: TxCreateWallet,
    ) -> Self::Output {
        let author = context
            .caller()
            .author()
            .expect("Wrong 'TxCreateWallet' initiator");

        let mut schema = CurrencySchema::new(context.service_data());
        if schema.wallets.get(&author).is_none() {
            let wallet = Wallet::new(&author, &arg.name, INIT_BALANCE);
            println!("Created wallet: {:?}", wallet);
            schema.wallets.put(&author, wallet);
            Ok(())
        } else {
            Err(Error::WalletAlreadyExists.into())
        }
    }

    fn transfer(
        &self,
        context: ExecutionContext<'_>,
        arg: TxTransfer,
    ) -> Self::Output {
        let author = context
            .caller()
            .author()
            .expect("Wrong 'TxTransfer' initiator");
        if author == arg.to {
            return Err(Error::SenderSameAsReceiver.into());
        }

        let mut schema = CurrencySchema::new(context.service_data());
        let sender = schema.wallets.get(&author).ok_or(Error::SenderNotFound)?;
        let receiver = schema
            .wallets
            .get(&arg.to)
            .ok_or(Error::ReceiverNotFound)?;

        let amount = arg.amount;
        if sender.balance >= amount {
            let sender = sender.decrease(amount);
            let receiver = receiver.increase(amount);
            println!("Transfer between wallets: {:?} => {:?}", sender, receiver);
            schema.wallets.put(&author, sender);
            schema.wallets.put(&arg.to, receiver);
            Ok(())
        } else {
            Err(Error::InsufficientCurrencyAmount.into())
        }
    }

}
