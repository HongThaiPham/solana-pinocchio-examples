// Helper function to create padded arrays of the right size
pub fn create_padded_array<const N: usize>(data: &[u8], size: usize) -> [u8; N] {
    let mut result = [0u8; N];
    let copy_size = data.len().min(size).min(N);
    result[..copy_size].copy_from_slice(&data[..copy_size]);
    result
}
