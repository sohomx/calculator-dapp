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
        let Calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1*num2;
        Ok(())
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }

}

// #[derive(Accounts)]
// pub struct Initialize {}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub Calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub Calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Substraction<'info> {
    #[account(mut)]
    pub Calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub Calculator: Account<'info, Calculator>
}


#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub reminder: i64   
}