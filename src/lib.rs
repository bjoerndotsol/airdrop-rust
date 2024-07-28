

#[cfg(test)]
mod tests {
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, pubkey::Pubkey};
    use bs58;
    use std::io::{self, BufRead, Stdin};

    #[test]
    fn keygen() {
        let kp: Keypair = Keypair::new();
        println!("You've generated a new wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file: ");
        println!("{:?}", kp.to_bytes());
    }    

    #[test]
    fn airdrop() {}
    #[test]
    fn transfer_sol() {}

    #[test]
    fn base64_to_wallet(){
        println!("Input private key as base 58:");
        let stdin: Stdin = io::stdin();
        let base58: String = stdin.lock().lines().next().unwrap().unwrap();
        let wallet: Vec<u8> = bs58::decode(base58).into_vec().unwrap();
        println!("Your wallet file is: {:?}", wallet);
        // // println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base64(){
        println!("Input your private key as a wallet file byt array: ");
        let stdin: Stdin= io::stdin();
        let wallet: Vec<u8> = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches("[").trim_end_matches("]").split(",").map(|s: &str| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        let base58: String = bs58::encode(wallet).into_string();
        println!("Your private key as a base 58 string is: {:?}", base58);
    }
}
