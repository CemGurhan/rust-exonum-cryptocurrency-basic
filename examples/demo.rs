use exonum_cli::{NodeBuilder, Spec};
use cryptocurrency::CryptocurrencyService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    exonum::helpers::init_logger()?;

    NodeBuilder::development_node()?
        // Starts cryptocurrency instance with given id and name
        // immediately after genesis block creation.
        .with(Spec::new(CryptocurrencyService).with_default_instance())
        .run().await
        
}
