use byteorder::{ByteOrder, LittleEndian};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};



pub fn process_instruction/*we can use our own types by declaring them here in the '<>'*/(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    )->ProgramResult{
   
    let accounts_iter=&mut accounts.iter();
    let target=next_account_info(accounts_iter);
   
    println!("{:?}", accounts_iter);
    println!("{:?}", target);

    //check ownership
    if target.owner!=program_id{
        info!("target account does not have the correct program id");

        return Err(ProgramError::IncorrectProgramId);
    }

    if !target.is_signer{
        info!("Target account must be signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let chunk_num=LittleEndian::read_u16(&instruction_data[0..2]);
}

pub fn upload_file(){
    println!("file upload");
}

#[derive(Debug)]
pub struct MyStruct {
    Name: String,
}

// entrypoint!(process_instruction);