use winit::keyboard::{KeyCode, PhysicalKey};

/// Maps physical keyboard keys to Chip8 keys
pub fn map_to_chip8(keycode: PhysicalKey) -> Option<u8> {
    match keycode {
        PhysicalKey::Code(KeyCode::Digit1) => Some(0x1),
        PhysicalKey::Code(KeyCode::Digit2) => Some(0x2),
        PhysicalKey::Code(KeyCode::Digit3) => Some(0x3),
        PhysicalKey::Code(KeyCode::Digit4) => Some(0xc),
        PhysicalKey::Code(KeyCode::KeyQ) => Some(0x4),
        PhysicalKey::Code(KeyCode::KeyW) => Some(0x5),
        PhysicalKey::Code(KeyCode::KeyE) => Some(0x6),
        PhysicalKey::Code(KeyCode::KeyR) => Some(0xd),
        PhysicalKey::Code(KeyCode::KeyA) => Some(0x7),
        PhysicalKey::Code(KeyCode::KeyS) => Some(0x8),
        PhysicalKey::Code(KeyCode::KeyD) => Some(0x9),
        PhysicalKey::Code(KeyCode::KeyF) => Some(0xe),
        PhysicalKey::Code(KeyCode::KeyZ) => Some(0xa),
        PhysicalKey::Code(KeyCode::KeyX) => Some(0x0),
        PhysicalKey::Code(KeyCode::KeyC) => Some(0xb),
        PhysicalKey::Code(KeyCode::KeyV) => Some(0xf),
        _ => None,
    }
}
