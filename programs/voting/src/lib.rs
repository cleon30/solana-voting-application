use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod voting {
    use super::*;
    ///init ///
    pub fn initialize(ctx: Context<Create>, ticket: u64) -> Result<()> {
        let voting: &mut Account<Voting> = &mut ctx.accounts.voting;        
        voting.authority = ctx.accounts.admin.key(); 
        voting.claimed = 0;               
        voting.countyes = 0;
        voting.countno = 0;            
        voting.ticket = voting;
        voting.oracle = oracle_pubkey;

        Ok(())
    }
    ///get a ticket////
    pub fn get_ticket(ctx: Context<Submit>) ->Result<()>{

    }

    //Oracle checks if today It'S before than Deadline ////

    // 

    //
}
////// CONTEXTS///////////

//////////////////////////////////
#[derive(Accounts)]

////////////////////

/// Accounts///////////////


