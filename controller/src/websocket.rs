/// Convert string of bytes to a websocket frame.
///
/// - `bytes`: the byte string to convert.
/// - `masking_key`: random 4 bytes to mask the payload.
pub fn bytes_to_websocket_frame<const N: usize>(
    bytes: &[u8; N],
    masking_key: [u8; 4],
) -> Result<[u8; N + 6], &'static str>
where
    [u8; N + 6]: Sized,
{
    let mut frame = [0; N + 6];

    // FIN + Opcode
    frame[0] = 0x81;

    // MASK + Payload Length
    if N > 125 {
        return Err("Payload length is too long"); // limit for single-byte length
    }

    frame[1] = 0x80 | (N as u8); // 0x80 for MASK bit

    // Masking Key
    frame[2..6].copy_from_slice(&masking_key);

    // Masked Payload
    for i in 0..N {
        frame[6 + i] = bytes[i] ^ masking_key[i % 4];
    }

    Ok(frame)
}
