mod programs;

#[cfg(test)]
mod tests {
    use solana_sdk::{hash::Hash, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signature, Signer}, transaction::Transaction, message::Message,};
    use solana_program::{pubkey, system_instruction::transfer, system_program};
    use solana_client::rpc_client::RpcClient;
    use bs58;
    use dotenv::dotenv;
    use std::env;
    use std::io::{self, BufRead, Stdin};
    use std::str::FromStr;
    use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};

    // RPC URL for devnet connection
    const RPC_URL: &str = "https://api.devnet.solana.com";

    // Generate a new wallet (keypair)
    #[test]
    fn keygen() {
        // Generate a new keypair
        let kp: Keypair = Keypair::new();

        // Print the public key
        println!("You've generated a new wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file: ");
        println!("{:?}", kp.to_bytes());
    }    

    // Request an airdrop in SOL to devnet wallet
    #[test]
    fn airdrop() {
        // Read the wallet file
        let kp: Keypair = read_keypair_file("dev-wallet.json").expect("Could not find wallet file");

        // Create a new RPC client
        let client: RpcClient = RpcClient::new(RPC_URL);

        // Request an 2 SOL airdrop
        match client.request_airdrop(&kp.pubkey(),  2_000_000_000u64) {
            // If the airdrop is successful, print the transaction signature
            Ok(s) => {
                println!("Success, check your TX here: ");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s);
            },
            // If the airdrop is not successful, print the error
            Err(e) => {
                println!("Failed to request airdrop: {}", e);
            }
        }
    }

    // Empty devnet wallet by sending remaining SOL to WBA wallet
    #[test]
    fn transfer_sol() {
        // Load the .env file
        dotenv().ok();

        // Read the wallet file from devnet wallet
        let kp: Keypair = read_keypair_file("dev-wallet.json").expect("Could not find wallet file");

        // Read the WBA public key from the .env file
        let public_key = env::var("PUBLIC_KEY").expect("PUBLIC_KEY not set");
        println!("Your public key is: {}", public_key); 
        let to_pubkey: Pubkey = Pubkey::from_str(&public_key).unwrap();

        // Create a new RPC client
        let rpc_client: RpcClient = RpcClient::new(RPC_URL);

        // Get the latest blockhash
        let recent_blockhash: Hash = rpc_client.get_latest_blockhash().expect("Could not get recent blockhash");

        // Get the balance of the devnet wallet
        let balance: u64 = rpc_client.get_balance(&kp.pubkey()).expect("Could not get balance");
        println!("Your balance is: {}", balance);

        // Create a new message to check the fee for the transaction
        let message: Message = Message::new_with_blockhash(&[transfer(&kp.pubkey(), &to_pubkey, balance)], Some(&kp.pubkey()), &recent_blockhash);

        // Get the fee for the transaction
        let fee: u64 = rpc_client.get_fee_for_message(&message).expect("Could not get fee for message");
        println!("The fee for the transaction is: {}", fee);

        // Create a new transaction to transfer SOL to WBA wallet
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[transfer(&kp.pubkey(), &to_pubkey, balance - fee)],
            Some(&kp.pubkey()),
            &vec![&kp],
            recent_blockhash,
        );

        // Send the transaction and confirm it
        let signature: Signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Error sending transaction");

        // Print the transaction signature
        println!("Success, check your TX here: ");
        println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

    // Convert a base 58 string to a wallet file
    #[test]
    fn base64_to_wallet(){
        // Print the input prompt
        println!("Input private key as base 58:");

        // Read the input from the user
        let stdin: Stdin = io::stdin();
        let base58: String = stdin.lock().lines().next().unwrap().unwrap();

        // Convert the base 58 string to a wallet file
        let wallet: Vec<u8> = bs58::decode(base58).into_vec().unwrap();

        // Print the wallet file
        println!("Your wallet file is: {:?}", wallet);
    }

    // Convert a wallet file to a base 58 string
    #[test]
    fn wallet_to_base64(){
        // Print the input prompt
        println!("Input your private key as a wallet file byt array: ");

        // Read the input from the user
        let stdin: Stdin= io::stdin();
        let wallet: Vec<u8> = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches("[").trim_end_matches("]").split(",").map(|s: &str| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();

        // Convert the wallet file to a base 58 string
        let base58: String = bs58::encode(wallet).into_string();

        // Print the base 58 string
        println!("Your private key as a base 58 string is: {:?}", base58);
    }

    // Enroll to the WBA prereq program
    #[test]
    fn enroll_to_program() {
        // Create a new RPC client
        let rpc_client: RpcClient = RpcClient::new(RPC_URL);

        // Read the wallet file from WBA wallet
        let signer: Keypair = read_keypair_file("wba-wallet.json").expect("Could not find wallet file");

        // Create a new Pubkey for the prereq program
        let prereq: Pubkey = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        // Set Github username
        let args: CompleteArgs = CompleteArgs{github: b"bjoerndotsol".to_vec()};

        // Get the latest blockhash
        let blockhash: Hash = rpc_client.get_latest_blockhash().expect("Could not get latest blockhash");

        // Create a new transaction to enroll to the prereq program
        let transaction: Transaction = WbaPrereqProgram::complete(&[&signer.pubkey(), &prereq, & system_program::id()], &args, Some(&signer.pubkey()), &[&signer], blockhash); 

        // Send the transaction and confirm it
        let signature: Signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Error sending transaction");

        // Print the transaction signature
        println!("Success, check your TX here: ");
        println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
}
