use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use solana_account_decoder::parse_token::get_token_account_mint;

use crate::instruction::CounterInstruction;
use crate::state::Counter;

//error
pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

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
            CounterInstruction::Increment => {
                //msg!("Instruction: Increment");
                let accounts_iter = &mut accounts.iter();

                let payer_account_info = next_account_info(accounts_iter)?;
                //msg!("payer_account_info sent {:?}",payer_account_info);

                let pda_account_info = next_account_info(accounts_iter)?;
                //msg!("pda_account_info sent {:?}",pda_account_info);

                let sys_var = next_account_info(accounts_iter)?;
               // msg!("sys_var sent {:?}",sys_var);

                let system_program = next_account_info(accounts_iter)?;
                //msg!("system_program sent {:?}",system_program);

                let user_token = next_account_info(accounts_iter)?;
                msg!("user_token {:?}",user_token);

                let spl_token = next_account_info(accounts_iter)?;
                msg!("spl_token {:?}",spl_token);

                let token_account_mint = solana_account_decoder::parse_token::get_token_account_mint(&user_token.data.borrow());
                


               

                //create pda account
                // let (pda_info, bump) = Pubkey::find_program_address(&[b"crankpay_seed"], _program_id);

                // msg!("pda_info {:?}",pda_info);

                // msg!("bump {:?}",bump);

                // solana_program::program::invoke_signed(

                //     //instruction
                //     &solana_program::system_instruction::create_account(
                //         payer_account_info.key,
                //         pda_account_info.key,
                //         Rent::get()?.minimum_balance(8),
                //         8,
                //         _program_id,
                //     ),
                //     //accounts_info
                //     &[payer_account_info.clone(), pda_account_info.clone(), system_program.clone()],

                //     //signers_seeds
                //     &[&[b"crankpay_seed", &[bump]]],
                // )?;






 


            }
        }
        Ok(())
    }
}
