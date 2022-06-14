use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::instruction::transfer;
use solana_program::program::invoke;
use spl_token::state::Account as TokenAccount;
use solana_program::program_pack::Pack;

use crate::instruction::CounterInstruction;
use crate::state::Escrow;


pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            
            CounterInstruction::InitEscrow{amount}=> {
                msg!("Instruction: Init Escrow");

                //pda 
                msg!("1st Seed used is {:?}",&[b"escrow"]);
                msg!("2nd Seed used is {:?}",_program_id);
                let (pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], _program_id);
                msg!("Program PDA is {}",pda);

                //data passed
                //Program logged: "Program PDA is EfR6jhPA9P89YxLnFGRAXkganxT5WiFAUpQ59geTHSEt"
                msg!("Data sent {}",amount);
                //Program logged: "Data sent 100"

                let accounts_iter = &mut accounts.iter();

                let feepayer = next_account_info(accounts_iter)?;
                msg!("FeePayer sent {:?}",feepayer);

                let spl_token = next_account_info(accounts_iter)?;
                msg!("Spl Token sent {:?}",spl_token);

                let token_program_id = next_account_info(accounts_iter)?;
                msg!("TokenProgram sent {:?}",token_program_id);

                let pda_account = next_account_info(accounts_iter)?;
                msg!("PDA sent {:?}",pda_account);

                let system_program = next_account_info(accounts_iter)?;
                msg!("system_program sent {:?}",system_program);
                
                // { key: DgeNUoaYCr5dQrDuCWp1SwgZ4um9NR46xqCNwGyd9EF6, owner: 11111111111111111111111111111111, is_signer: true, is_writable: true, executable: false, rent_epoch: 323, lamports: 25486670200, data.len: 0, .. }

                
                //create pda account 
                let pda_account_ix = solana_program::system_instruction::create_account(
                    feepayer.key, 
                    pda_account.key, 
                    Rent::get()?.minimum_balance(TokenAccount::LEN), // enough sol to pay for rent 
                    TokenAccount::LEN as u64, //space to hold meta data
                    _program_id,
                );

                msg!("TokenProgram sent {:?}",pda_account_ix);

                let res = solana_program::program::invoke_signed(
                    &pda_account_ix,
                    &[
                        feepayer.clone(),
                        pda_account.clone(),
                        system_program.clone()                   
                    ],
                    &[&[b"escrow"],&[&[_nonce]]]
                )?;

                msg!("Res:  {:?}",res);

  



            }
        }
        Ok(())
    }
}
