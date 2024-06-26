pub const BLS12_381_G1_POINT_LEN: usize = 48;
pub const BLS12_381_G2_POINT_LEN: usize = 96;

/// A generator in G1 (in compressed serialization).
///
/// This can be used directly for signature verification
/// (see e.g. https://twitter.com/simon_warta/status/1786342207106019765)
pub const BLS12_381_G1_GENERATOR: [u8; BLS12_381_G1_POINT_LEN] = [
    151, 241, 211, 167, 49, 151, 215, 148, 38, 149, 99, 140, 79, 169, 172, 15, 195, 104, 140, 79,
    151, 116, 185, 5, 161, 78, 58, 63, 23, 27, 172, 88, 108, 85, 232, 63, 249, 122, 26, 239, 251,
    58, 240, 10, 219, 34, 198, 187,
];

/// A generator in G2 (in compressed serialization).
///
/// This can be used directly for signature verification
/// (see e.g. https://twitter.com/simon_warta/status/1786342207106019765)
pub const BLS12_381_G2_GENERATOR: [u8; BLS12_381_G2_POINT_LEN] = [
    147, 224, 43, 96, 82, 113, 159, 96, 125, 172, 211, 160, 136, 39, 79, 101, 89, 107, 208, 208,
    153, 32, 182, 26, 181, 218, 97, 187, 220, 127, 80, 73, 51, 76, 241, 18, 19, 148, 93, 87, 229,
    172, 125, 5, 93, 4, 43, 126, 2, 74, 162, 178, 240, 143, 10, 145, 38, 8, 5, 39, 45, 197, 16, 81,
    198, 228, 122, 212, 250, 64, 59, 2, 180, 81, 11, 100, 122, 227, 209, 119, 11, 172, 3, 38, 168,
    5, 187, 239, 212, 128, 86, 200, 193, 33, 189, 184,
];
