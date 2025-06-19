use pinocchio::program_error::ProgramError;

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy)]
pub struct MintAuthority {
    pub bump: u8,
}

impl MintAuthority {
    pub const SEED_PREFIX: &[u8] = b"mint_authority";
    pub const LEN: usize = core::mem::size_of::<MintAuthority>();

    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != MintAuthority::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != MintAuthority::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    #[inline(always)]
    pub fn set_inner(&mut self, bump: u8) {
        self.bump = bump;
    }
}
