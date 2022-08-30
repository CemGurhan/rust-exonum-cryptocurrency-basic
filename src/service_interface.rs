// use crate::proto::*;
use exonum_derive::*;
use exonum_rust_runtime::{api::ServiceApiBuilder, DefaultInstance, Service};
use exonum::runtime::{ExecutionContext, ExecutionError};
use crate::error::*;
use crate::schema::*;
use crate::wallets::*;
use crate::transactions::*;
use crate::cryptoapi::CryptocurrencyApi;
const INIT_BALANCE: u64 = 100;

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

/// Cryptocurrency service implementation.
#[derive(Debug, ServiceFactory, ServiceDispatcher)]
#[service_dispatcher(implements("CryptocurrencyInterface"))]
#[service_factory(proto_sources = "crate::proto")]
pub struct CryptocurrencyService;

impl Service for CryptocurrencyService {

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        CryptocurrencyApi::wire(builder);
    }

}

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

