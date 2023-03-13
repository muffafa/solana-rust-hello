pub mod instruction;
use instruction::MovieInstruction;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data)?;

    match instruction {
        MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => add_movie_review(program_id, accounts, title, rating, description),
    }
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("MUFFAFA'NIN PROGRAMINA HOŞGELDİNİZ!");
    msg!("İSİM: {}", title);
    msg!("PUAN 5 ÜZERİNDEN: {}", rating);
    msg!("AÇIKLAMA: {}", description);
    msg!("KILIÇDAROĞLU ADAY OLMASIN!");
    Ok(())
}