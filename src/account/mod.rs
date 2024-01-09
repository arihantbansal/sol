use crate::{utils::get_network, Account};

use prettytable::{row, Table};
use solana_account_decoder::{
    parse_account_data::{parse_account_data, ParsedAccount as ParsedAccountData},
    UiAccountEncoding,
};
use solana_client::{
    nonblocking::rpc_client::RpcClient as Client, rpc_config::RpcAccountInfoConfig,
};
use solana_sdk::{
    account::Account as SolanaAccount, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    system_program,
};
use spl_token;

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

    let parsed_account = parse_account(&account.pubkey, fetched_account, &client)
        .await
        .unwrap();

    parsed_account.view();
}

async fn parse_account(
    pubkey: &Pubkey,
    account: SolanaAccount,
    client: &Client,
) -> Option<ParsedAccount> {
    let SolanaAccount {
        lamports,
        data,
        owner,
        executable,
        rent_epoch,
    } = account;

    let parsed_data = if data.len() != 0 {
        let unwrapped_data = parse_account_data(pubkey, &owner, &data, None).unwrap();
        Some(unwrapped_data)
    } else {
        // TODO
        // user data
        None
    };

    let name = match owner {
        system_program::ID => Some("System Program".to_string()),
        spl_token::ID => Some("Token Program".to_string()),
        _ => None,
    };

    Some(ParsedAccount {
        owner: Owner {
            name,
            pubkey: owner,
        },
        lamports,
        parsed_data,
        executable,
    })
}

pub struct Owner {
    name: Option<String>,
    pubkey: Pubkey,
}

pub struct ParsedAccount {
    owner: Owner,
    lamports: u64,
    parsed_data: Option<ParsedAccountData>,
    executable: bool,
}

impl ParsedAccount {
    fn view(self) {
        let mut status_table = Table::new();
        status_table.set_titles(row![
            c-> "Account Overview",
        ]);

        let owner = if self.owner.name.is_some() {
            self.owner.name.unwrap()
        } else {
            self.owner.pubkey.to_string()
        };

        let sol_balance = (self.lamports as f64) / (LAMPORTS_PER_SOL as f64);

        status_table.add_row(row!["Owner", owner]);
        status_table.add_row(row!["SOL Balance", format!("{:.2}", sol_balance)]);

        let mut table_of_tables = Table::new();
        table_of_tables.add_row(row![c->status_table]);
        table_of_tables.printstd();
    }
}
