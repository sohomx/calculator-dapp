use anchor_lang::prelude::*;

declare_id!("A6Gg6sKdWF4qbSLTa13iBeVkFPRGSsLSurAwYMFsMu3G");

#[program]
pub mod myepicproject {
    use super::*;
    // pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    //     Ok(())
    // }

    pub fn create(ctx:Context<Create>, init_message: String) -> ProgramResult {
        let caluclator = &mut ctx.accounts.caluclator;
        caluclator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {

    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {

    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {

    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {

    }

}

// #[derive(Accounts)]
// pub struct Initialize {}
