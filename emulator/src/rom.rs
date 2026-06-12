use std::{fs, path::Path};

/// Reads ROM bytes.
pub fn read_rom_bytes<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<u8>> {
    let path = path.as_ref();

    let rom_bytes: Vec<u8> = fs::read(&path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read ROM bytes from path '{}': {}",
            path.display(),
            e
        )
    })?;

    Ok(rom_bytes)
}
