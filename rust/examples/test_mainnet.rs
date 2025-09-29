use tx_parse::TxParseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TxParseClient::new("https://fullnode.mainnet.sui.io:443");

    let tx_digest = "J5BzQREx52w3t75bFSZAy3uRpGne543vx251ZDf6LKmR";
    let bag_id = "0x64ac48a57c8dfb3f69d5b0956be0c6727267978a11a53659c71f77c13c58aaad";

    println!("Testing bag dynamic field balance changes on Sui mainnet...");
    println!("Transaction: {}", tx_digest);
    println!("Bag ID: {}", bag_id);
    println!();

    let changes = client
        .get_bag_dynamic_field_balance_changes(tx_digest, bag_id)
        .await?;

    println!("Found {} dynamic field balance changes:\n", changes.len());

    if changes.is_empty() {
        println!("No balance changes found for this bag in this transaction.");
    } else {
        for (index, change) in changes.iter().enumerate() {
            let prev_formatted = change.previous_value.parse::<f64>().unwrap_or(0.0)
                / 10_f64.powi(change.decimals as i32);
            let curr_formatted = change.current_value.parse::<f64>().unwrap_or(0.0)
                / 10_f64.powi(change.decimals as i32);
            let diff_formatted = change.value_diff.parse::<f64>().unwrap_or(0.0)
                / 10_f64.powi(change.decimals as i32);

            println!("Change #{}:", index + 1);
            println!("  Coin Type: {}", change.coin_type);
            println!("  Decimals: {}", change.decimals);
            println!(
                "  Previous Value: {} ({:.9})",
                change.previous_value, prev_formatted
            );
            println!(
                "  Current Value: {} ({:.9})",
                change.current_value, curr_formatted
            );
            println!("  Difference: {} ({:.9})", change.value_diff, diff_formatted);
            println!();
        }
    }

    Ok(())
}