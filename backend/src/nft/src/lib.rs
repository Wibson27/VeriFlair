use ic_cdk::export_candid;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}! This is the NFT canister.", name)
}

export_candid!();