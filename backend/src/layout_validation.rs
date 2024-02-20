const REQUIRED_CHARS: &str = "abcdefghijklmnopqrstuvwxyz'";
pub const DEFAULT_LAYOUT_DATA: &str = "qwertyuiopasdfghjkl;'zxcvbnm,./"; // QWERTY

pub fn validate_layout_data(layout_data: &str) -> bool {
    if layout_data.len() != 31 {
        return false;
    }

    let mut counts = [0; 256];
    for b in layout_data.bytes() {
        let c = b as char;
        // invalid chars
        if !is_valid_layout_char(c) {
            return false;
        }

        counts[b as usize] += 1;
    }

    // check character completeness + unique
    for b in REQUIRED_CHARS.bytes() {
        if counts[b as usize] != 1 {
            return false;
        }
    }

    true
}

fn is_valid_layout_char(c: char) -> bool {
    if c.is_ascii_lowercase() {
        return true;
    }

    c == ';' || c == ':' || c == ',' || c == '.' || c == '/' || c == '\'' || c == '"' || c == '[' || c == ']'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_layout_data() {
        assert_eq!(validate_layout_data("qwertyuiopasdfghjkl;'zxcvbnm,./"), true); // qwerty
        assert_eq!(validate_layout_data("qwfpbjluy;arstgmneio'zxcdvkh,./"), true); // colemak dh
        assert_eq!(validate_layout_data("ypoujkdlcwinea,mhtsr'qz/.:bfgvx"), true); // mtgap
        assert_eq!(validate_layout_data("qwfpbjluy;arstgmneio'zxcdvkh,."), false); // invalid
        assert_eq!(validate_layout_data(DEFAULT_LAYOUT_DATA), true);
    }
}