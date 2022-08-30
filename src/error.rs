

use exonum_derive::*;



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