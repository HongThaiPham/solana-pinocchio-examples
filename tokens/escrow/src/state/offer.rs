use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy)]
pub struct Offer {
    pub id: [u8; 8],
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: [u8; 8],
    pub bump: u8,
}

impl Offer {
    pub const SEED_PREFIX: &'static [u8] = b"offer";
    pub const LEN: usize = core::mem::size_of::<Offer>();

    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Offer::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != Offer::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    #[inline(always)]
    pub fn set_inner(
        &mut self,
        id: [u8; 8],
        maker: Pubkey,
        token_mint_a: Pubkey,
        token_mint_b: Pubkey,
        token_b_wanted_amount: [u8; 8],
        bump: u8,
    ) {
        self.id = id;
        self.maker = maker;
        self.token_mint_a = token_mint_a;
        self.token_mint_b = token_mint_b;
        self.token_b_wanted_amount = token_b_wanted_amount;
        self.bump = bump;
    }
}
