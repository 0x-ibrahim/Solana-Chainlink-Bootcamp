use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub name: String,
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("GM program entrypoint");

    let accounts = &mut accounts.iter();

    let account = next_account_info(accounts)?;

    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the input data, and store it in GreetingAccount struct.
    // let mut greeting_account = GreetingAccount::try_from_slice(&input).unwrap();
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    greeting_account.counter += 1;
    msg!("GM {}", greeting_account.name);
    msg!("GM {}", greeting_account.counter);

    // serialize the name and store it in passed in account.
    greeting_account.serialize(&mut &mut account.try_borrow_mut_data()?[..])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
