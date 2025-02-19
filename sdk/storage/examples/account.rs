use azure_storage::core::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_client = StorageClient::new_access_key(&account, &access_key);

    let account = storage_client
        .get_account_information()
        .into_future()
        .await?;
    println!("Account info:");
    println!("\tKind: {}", account.account_kind);
    println!("\tSku: {}", account.sku_name);

    Ok(())
}
