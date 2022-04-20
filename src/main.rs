use anyhow::Error;
use utility::testing::get_all_sample_data;

#[tokio::main]
async fn main() -> Result<(), Error> {
    get_all_sample_data().await?;
    Ok(())
}
