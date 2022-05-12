use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod voting {
    use super::*;
    ///init ///
    pub fn initialize(ctx: Context<Initialize>, ticket: u64) -> Result<()> {
        let voting: &mut Account<Voting> = &mut ctx.accounts.voting;        
        voting.authority = ctx.accounts.admin.key(); 
        voting.claimed = 0;               
        voting.countyes = 0;
        voting.countno = 0;            
        voting.ticket = voting;
        voting.oracle = oracle_pubkey;
        voting.winner = winner;

        Ok(())
    }
    
    ///get a ticket////
    pub fn get_ticket(ctx: Context<Claim>) ->Result<()>{


    }
    pub fn yes(ctx: Context) -> Result<()>{

    }
    pub fn no(ctx: Context) -> Result<()>{

    }

    //Oracle checks if today It'S before than Deadline ////

    // 

    //
}
////// CONTEXTS///////////
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 180)]
    pub voting: Account<'info, Voting>,
    #[account(mut)]
    pub admin: Signer<'info>,    
    pub system_program: Program<'info, System>,

    pub struct Claim<'info>{

    }
}

//////////////////////////////////
#[derive(Accounts)]

////////////////////

/// Accounts//////////////
//Errors/
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}