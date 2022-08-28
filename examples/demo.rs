use exonum_cli::NodeBuilder;
use failure::Error;

use exonum_cryptocurrency::contracts::CryptocurrencyService;

fn main() -> Result<(), Error> {
    exonum::helpers::init_logger()?;
    NodeBuilder::development_node()?
        .with_default_rust_service(CryptocurrencyService)
        .run()
}
