use anchor_lang::prelude::*;

declare_id!("3ZjPdPuMrSGjv6WKTZJNDC63fxpww5sgV9GPyVZBDGch");

#[program]
pub mod membership_club {
    use super::*;

    // Paso 1: Creas tu "Perfil de Miembro"
    pub fn create_profile(ctx: Context<CreateProfile>, name: String) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        profile.name = name;
        profile.level = 1; // Todos empiezan en nivel 1
        profile.authority = *ctx.accounts.signer.key;
        
        msg!("¡Perfil creado para {}! Bienvenido al club.", profile.name);
        Ok(())
    }

    // Paso 2: Intentas subir de nivel (Solo si eres el dueño)
    pub fn upgrade_level(ctx: Context<UpdateProfile>) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        
        // validación
        if profile.level >= 10 {
            msg!("¡Ya eres nivel máximo!");
            return Ok(()); 
        }

        profile.level += 1;
        msg!("¡Felicidades {}! Ahora eres nivel {}.", profile.name, profile.level);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    // Espacio: 8 (disc) + 32 (String) + 1 (u8) + 32 (Pubkey)
    #[account(init, payer = signer, space = 8 + 32 + 1 + 32)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut, has_one = authority)] // "has_one" verifica que el firmante sea el dueño
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub name: String,     // Nombre del usuario
    pub level: u8,        // Nivel (u8 ocupa solo 1 byte, del 0 al 255)
    pub authority: Pubkey, // Quién tiene permiso de editar esto
}