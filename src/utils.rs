
pub fn __stringify_vecu8_to_binary(vecu8: &[u8]) -> Vec<String> {
    vecu8.iter().map(|u| format!("{:#010b}", u)).collect()
}

pub fn __stringify_vecu8_to_hex(vecu8: &[u8]) -> Vec<String> {
    vecu8.iter().map(|u| format!("{:#04X}", u)).collect()
}