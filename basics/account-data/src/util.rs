use pinocchio::program_error::ProgramError;

#[inline(always)]
pub unsafe fn load_acc_mut_unchecked<T>(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
    if bytes.len() != core::mem::size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}
