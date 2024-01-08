use crate::{utils::get_network, Account};

use prettytable::{row, Table};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient as Client, rpc_config::RpcAccountInfoConfig,
};
use solana_sdk::account::Account as SolanaAccount;

pub async fn handler(rpc_url: String, account: Account) {
    // Build RPC client
    let client = Client::new(get_network(&rpc_url));

    let fetched_account = client
        .get_account_with_config(
            &account.pubkey,
            RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base58),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .value
        .unwrap();

    let parsed_account = parse_account(fetched_account, &client).await.unwrap();

    parsed_account.view();
}

async fn parse_account(account: SolanaAccount, client: &Client) -> Option<ParsedAccount> {
    Some(ParsedAccount {})
}

pub struct ParsedAccount {}

impl ParsedAccount {
    fn view(self) {
        let mut status_table = Table::new();
        status_table.set_titles(row![
            c-> "Account Overview",
        ]);
    }
}
