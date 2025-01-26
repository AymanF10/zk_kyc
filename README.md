## Overview
This project implements a KYC (Know Your Customer) verification system on the Solana blockchain using Rust. The program utilizes zero-knowledge proofs (ZKP) to verify if a user has sufficient balance without revealing the actual balance. The client interacts with the Solana blockchain to send transactions and invoke the KYC logic.

## Table of Contents
- [Installation](#installation)
- [Project Structure](#project-structure)
- [Program Logic](#program-logic)
  - [KYC Proof Generation](#kyc-proof-generation)
  - [Client Interaction](#client-interaction)
- [Usage](#usage)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/solana-kyc-verification.git
   cd solana-kyc-verification
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run --bin client
   ```

## Project Structure
The project consists of several key files:

| File                  | Description                                                   |
|-----------------------|---------------------------------------------------------------|
| `client.rs`           | Contains the client logic for interacting with the Solana blockchain and sending transactions. |
| `lib.rs`              | Contains the KYC program logic, including proof generation and entry point for processing instructions. |
| `main.rs`             | Entry point for asynchronous execution, generating user keypairs and preparing to send transactions. |

## Program Logic

### KYC Proof Generation
The KYC program uses a simple structure to simulate generating a proof for KYC verification based on user balance.

```rust
#[derive(Debug)]
pub struct KycProof {
    pub balance_check: bool, // Indicates if the user has sufficient balance
}

// Function to simulate generating a ZKP for KYC verification
fn generate_kyc_proof(user_balance: u64, required_balance: u64) -> KycProof {
    KycProof {
        balance_check: user_balance >= required_balance,
    }
}
```

The `process_instruction` function serves as the entry point for handling incoming transactions, checking user balances, and generating KYC proofs.

### Client Interaction
The client communicates with the Solana blockchain to send transactions that invoke the KYC logic.

```rust
fn main() {     
    let program_id = Pubkey::from_str("zk_kyc").unwrap(); 
    let url = "http://localhost:8899".to_string(); 
    let client = RpcClient::new(url);

    // Load keypair from file     
    let payer = read_keypair_file("../my-keypair2.json").expect("Failed to read keypair file");    

    // Check balance and request airdrop if needed 
    let balance = client.get_balance(&payer.pubkey()).unwrap(); 
    if balance < 1_000_000_000 { 
        let signature = client.request_airdrop(&payer.pubkey(), 1_000_000_000 - balance).unwrap(); 
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
    println!("Transaction signature: {}", signature); 
}
```

## Usage
1. **Deploy the Program**: Deploy your program to the Solana devnet using:
   ```bash
   solana program deploy path/to/your/program.so
   ```

2. **Run the Client**: Execute the client to interact with your deployed program:
   ```bash
   cargo run --bin client
   ```

3. **Check Output**: The client will print transaction signatures and any relevant messages regarding KYC verification.

## Testing
You can add tests to ensure that your functions work as expected. Create a new file named `tests.rs` in the `src` directory and write your test cases there.

Run tests using:
```bash
cargo test
```
## License
This project is licensed under the MIT License - see the LICENSE file for details.
