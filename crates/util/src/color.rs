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

pub fn ref_addr<T: Sized>(t: &T) -> String {
    let addr = std::ptr::from_ref(t);
    ptr_inv(addr)
}
pub fn ref_addr_inv<T: Sized>(t: &T) -> String {
    let addr = std::ptr::from_ref(t);
    ptr(addr)
}
pub fn ref_mut_addr<T: Sized>(t: &mut T) -> String {
    let addr = std::ptr::from_mut(t);
    ptr_inv(addr)
}
pub fn ref_mut_addr_inv<T: Sized>(t: &mut T) -> String {
    let addr = std::ptr::from_mut(t);
    ptr(addr)
}
pub fn ptr_colors<T: Sized>(ptr: *const T) -> (u8, u8) {
    addr_colors(ptr.addr())
}
pub fn ptr_repr<T>(
    ptr: *const T,
    bg: u8,
    fg: u8,
    null_bg: u8,
    null_fg: u8,
    nonnull_bg: u8,
    nonnull_fg: u8,
) -> String {
    if ptr.is_null() {
        reset(bgfg("null", null_fg.into(), null_bg.into()))
    } else {
        addr_repr(ptr.addr(), bg, fg, null_bg, null_fg, nonnull_bg, nonnull_fg)
    }
}
pub fn ptr<T>(ptr: *const T) -> String {
    addr(ptr.addr())
}
pub fn ptr_inv<T>(ptr: *const T) -> String {
    addr_inv(ptr.addr())
}

pub fn addr_colors(addr: usize) -> (u8, u8) {
    match addr {
        0 => (255, 9),
        8 => (16, 137),
        addr => couple(addr),
    }
}

pub fn addr_repr(
    addr: usize,
    bg: u8,
    fg: u8,
    null_bg: u8,
    null_fg: u8,
    nonnull_bg: u8,
    nonnull_fg: u8,
) -> String {
    if addr == 0 {
        reset(bgfg("null", null_fg.into(), null_bg.into()))
    } else {
        format!(
            "{}{}{}",
            reset(bgfg(format!("0x{:016x}", addr), fg.into(), bg.into())),
            bgfg(":", 231, 16),
            if addr == 0 {
                reset(bgfg("null", null_fg.into(), null_bg.into()))
            } else {
                reset(bgfg("non-null", nonnull_fg.into(), nonnull_bg.into()))
            }
        )
    }
}
pub fn addr(addr: usize) -> String {
    let (bg, fg) = addr_colors(addr);
    let (null_bg, null_fg) = couple(9);
    let (nonnull_bg, nonnull_fg) = couple(101);
    addr_repr(addr, bg, fg, null_bg, null_fg, nonnull_bg, nonnull_fg)
}
pub fn addr_inv(addr: usize) -> String {
    let (fg, bg) = addr_colors(addr);
    let (null_fg, null_bg) = couple(9);
    let (nonnull_fg, nonnull_bg) = couple(101);
    addr_repr(addr, bg, fg, null_bg, null_fg, nonnull_bg, nonnull_fg)
}
