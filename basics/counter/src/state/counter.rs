use bytemuck::{Pod, Zeroable};
use pinocchio::program_error::ProgramError;

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Counter {
    pub count: [u8; 8],
}

impl Counter {
    pub const LEN: usize = core::mem::size_of::<Self>();

    pub fn set_inner(&mut self, data: Self) -> Self {
        self.count = data.count;
        self.clone()
    }
}

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy)]
pub enum MutationType {
    INCREASE,
    DECREASE,
}

impl TryFrom<&[u8]> for MutationType {
    type Error = ProgramError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        match data.first() {
            Some(0) => Ok(MutationType::INCREASE),
            Some(1) => Ok(MutationType::DECREASE),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
