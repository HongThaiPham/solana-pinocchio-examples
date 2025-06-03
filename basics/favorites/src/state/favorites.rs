use bytemuck::{Pod, Zeroable};

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Favorites {
    pub number: [u8; 8],
    pub color: [u8; 50],
    pub hobbies: [[u8; 50]; 5],
    pub bump: u8,
}

impl Favorites {
    pub const LEN: usize = core::mem::size_of::<Self>();

    pub fn set_inner(&mut self, data: Self) -> Self {
        self.number = data.number;
        self.color.copy_from_slice(&data.color);
        for (i, hobby) in data.hobbies.iter().enumerate() {
            self.hobbies[i].copy_from_slice(hobby);
        }
        self.bump = data.bump;
        *self
    }
}
