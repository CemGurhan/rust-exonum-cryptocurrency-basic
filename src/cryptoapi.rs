use exonum::crypto::{Hash, PublicKey};
use exonum::merkledb::{
    access::{Access, FromAccess},
    Fork, MapIndex, Snapshot,
};


use exonum_rust_runtime::api::{self, ServiceApiState};
use exonum_rust_runtime::{api::ServiceApiBuilder, DefaultInstance, Service};

use crate::wallets::Wallet;
use crate::schema::CurrencySchema;
use crate::transactions::*;
use crate::service_interface::CryptocurrencyService;


use serde_derive::{Deserialize, Serialize};

const INIT_BALANCE: u64 = 100;








#[derive(Debug, Clone, Copy)]
pub struct CryptocurrencyApi;

/// The structure describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WalletQuery {
    /// Public key of the requested wallet.
    pub pub_key: PublicKey,
}

impl CryptocurrencyApi {
    /// Endpoint for getting a single wallet.
    pub async fn get_wallet(
        state: ServiceApiState,
        query: WalletQuery
    ) -> api::Result<Wallet> {
        let schema = CurrencySchema::new(state.service_data());
        schema
            .wallets
            .get(&query.pub_key)
            .ok_or_else(|| api::Error::not_found().title("Wallet not found"))
    }

    /// Endpoint for dumping all wallets from the storage.
    pub async fn get_wallets(
        state: ServiceApiState,
        _query: ()
    ) -> api::Result<Vec<Wallet>> {
        let schema = CurrencySchema::new(state.service_data());
        Ok(schema.wallets.values().collect())
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        // Binds handlers to specific routes.
        builder
            .public_scope()
            .endpoint("v1/wallet", Self::get_wallet)
            .endpoint("v1/wallets", Self::get_wallets);
    }
}

type Handle<Query, Response> =
    fn(&ServiceApiState, Query) -> api::Result<Response>; // find the transaction endpoint idiomatic signiture !
impl DefaultInstance for CryptocurrencyService {  // import defualtinstance package (use default instance)
    const INSTANCE_ID: u32 = 101;
    const INSTANCE_NAME: &'static str = "cryptocurrency";
}
