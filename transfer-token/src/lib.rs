#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use solana_program:: {
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        program::invoke,
        pubkey::Pubkey,
        system_instruction,
        system_program
    };
    use solana_sdk::{signature::Keypair, signer::Signer};
    use std::fs;
    use std::fs::File;
    use std::io::{self, Read, Write};
    

    entrypoint!(process_instruction);

    #[test]
    fn create_keypair() {
        let file_path = "test-wallet.json";

        if fs::metadata(file_path).is_err() {
            let keypair = Keypair::new();
            println!("Output Keypair Public Key: {}, Secret Key: {:?}", keypair.pubkey(), keypair.secret());
            let file = File::create(file_path);
            file.unwrap().write_all(keypair.to_base58_string());
        }else {            
            let mut file = File::open(file_path);
            let mut contents = String::new();
            file.unwrap().read_to_string(&mut contents);
        }
    }


    #[test]
    fn request_airdrop() {

    }

    // #[test]
    // fn run_instruction() {
    //     let program_id = Pubkey::find_program_address(seeds, program_id)
    // }

    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        
        let payer = next_account_info(account_info_iter)?;
        let recipient = next_account_info(account_info_iter)?;

        assert!(payer.is_writable);
        assert!(payer.is_signer);
        assert!(recipient.is_writable);

        let lamports = 1000000;

        invoke(
            &system_instruction::transfer(payer.key, recipient.key, lamports),
            &[payer.clone(), recipient.clone()]
        )
        
    }
}