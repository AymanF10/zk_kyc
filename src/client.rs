use solana_client::rpc_client::RpcClient; 
use solana_sdk::signature::read_keypair_file; 
use solana_sdk::{ 
    commitment_config::CommitmentConfig, 
    instruction::{AccountMeta,Instruction},
    pubkey::Pubkey, 
    signature::Signer, 
    transaction::Transaction, 
}; 
use std::str::FromStr; 
use std::fs::File; 
use std::io::Read; 

fn main() {     
    let program_id = Pubkey::from_str("zk_kyc").unwrap(); 
    let url = "http://localhost:8899".to_string(); 
    let _client = RpcClient::new("http://localhost:8899".to_string());

    // Load keypair from file     
    let mut file = File::open("../YOUR_KEYPAIR.json").expect("Unable to open keypair file");     
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).expect("Unable to read keypair file");    
    // let payer = Keypair::from_base58_string(&contents); 
    let payer = read_keypair_file("../my-keypair2.json").expect("Failed to read keypair file");    

    // Check balance and request airdrop if needed 
    let balance = client.get_balance(&payer.pubkey()).unwrap(); 
    if balance < 1_000_000_000 { 
        let signature = client 
            .request_airdrop(&payer.pubkey(), 1_000_000_000 - balance) 
            .unwrap(); 
        client.confirm_transaction(&signature).unwrap(); 
        println!("Airdrop completed"); 
    } 

    let instruction = Instruction::new_with_borsh( 
        program_id, 
        &(), 
        vec![AccountMeta::new(payer.pubkey(), true)], // Changed to true for signing 
    ); 

    let recent_blockhash = client.get_latest_blockhash().unwrap(); 
    let transaction = Transaction::new_signed_with_payer( 
        &[instruction], 
        Some(&payer.pubkey()), 
        &[&payer], 
        recent_blockhash, 
    ); 

    let signature = client.send_and_confirm_transaction(&transaction).unwrap(); 
    println!("Hey guys! Transaction signature: {}", signature); 
}  