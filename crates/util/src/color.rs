use std::fmt::{Debug, Display};

pub fn fg<T: Display>(text: T, fg: usize) -> String {
    format!("\x1b[1;38;5;{}m{}", wrap(fg), text)
}
pub fn bg<T: Display>(text: T, bg: usize) -> String {
    format!("\x1b[1;48;5;{}m{}", wrap(bg), text)
}
pub fn reset<T: Display>(text: T) -> String {
    format!("{}\x1b[0m", text)
}
pub fn bgfg<T: Display>(text: T, fore: usize, back: usize) -> String {
    bg(fg(text, wrap(fore) as usize), wrap(back) as usize)
}
pub fn ansi<T: Display>(text: T, fore: usize, back: usize) -> String {
    reset(bgfg(text, fore as usize, back as usize))
}
pub fn ansi_clear() -> String {
    "\x1b[2J\x1b[3J\x1b[H".to_string()
}
pub fn fore<T: Display>(text: T, fore: usize) -> String {
    let (fore, back) = couple(fore);
    ansi(text, fore as usize, back as usize)
}
pub fn back<T: Display>(text: T, back: usize) -> String {
    let (back, fore) = couple(back);
    ansi(text, fore as usize, back as usize)
}
pub fn from_string<T: Display>(word: T) -> u8 {
    from_bytes(word.to_string().as_bytes())
}
pub fn rgb_from_string<T: Display>(word: T) -> [u8; 3] {
    rgb_from_bytes(word.to_string().as_bytes())
}
pub fn from_bytes(bytes: &[u8]) -> u8 {
    eprintln!("");
    let mut color: u8 = 0;
    for rgb in rgb_from_bytes(bytes) {
        color = color ^ rgb
    }
    color
}
pub fn rgb_from_bytes(bytes: &[u8]) -> [u8; 3] {
    let mut color: [u8; 3] = [0, 0, 0];
    for (index, byte) in bytes.iter().enumerate() {
        color[index % 3] = *byte
    }
    color
}

pub fn couple(color: usize) -> (u8, u8) {
    let fore = wrap(color);
    let back = invert_bw(fore);
    (fore, back)
}

pub fn invert_bw(color: u8) -> u8 {
    match color {
        0 | 8 | 16..21 | 52..61 | 88..93 | 232..239 => 231,
        _ => 16,
    }
}

pub fn wrap(color: usize) -> u8 {
    (if color > 0 {
        color % 255
    } else {
        color
    }) as u8
}
