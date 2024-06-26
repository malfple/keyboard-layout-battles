use crate::words::Wordlist;

const REQUIRED_CHARS: &str = "abcdefghijklmnopqrstuvwxyz'";
const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyz.,<>;:\'\"/?()[]{}-";
pub const DEFAULT_LAYOUT_DATA: &str = "qwertyuiopasdfghjkl;'zxcvbnm,./"; // QWERTY

pub fn validate_base_layout_data(layout_data: &str) -> bool {
    if layout_data.len() != 31 {
        return false;
    }

    let mut counts = [0; 256];
    for b in layout_data.bytes() {
        let c = b as char;
        // invalid chars
        if !ALLOWED_CHARS.contains(c) {
            return false;
        }

        counts[b as usize] += 1;
        // base layer must have unique characters
        if counts[b as usize] > 1 {
            return false;
        }
    }

    // check character completeness + unique
    // for b in REQUIRED_CHARS.bytes() {
    //     if counts[b as usize] != 1 {
    //         return false;
    //     }
    // }
    // no longer need required chars

    true
}

/// Calculates the difference between 2 layouts, only considers the required chars.
/// 
/// Value returned is between 0 and 1.
pub fn calc_layout_difference(wordlist: &Wordlist, layout_data_1: &str, layout_data_2: &str) -> f64 {
    let mut diff = 0.0;

    for b in REQUIRED_CHARS.bytes() {
        let c = b as char;
        if let Some(p1) = layout_data_1.find(c) {
            if let Some(p2) = layout_data_2.find(c) {
                if p1 != p2 {
                    diff += wordlist.char_freq[b as usize];
                }
            }
        }
    }

    diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_validate_base_layout_data() {
        assert_eq!(validate_base_layout_data("qwertyuiopasdfghjkl;'zxcvbnm,./"), true); // qwerty
        assert_eq!(validate_base_layout_data("qwfpbjluy;arstgmneio'zxcdvkh,./"), true); // colemak dh
        assert_eq!(validate_base_layout_data("ypoujkdlcwinea,mhtsr'qz/.:bfgvx"), true); // mtgap
        assert_eq!(validate_base_layout_data("qwfpbjluy;arstgmneio'zxcdvkh,."), false); // invalid
        assert_eq!(validate_base_layout_data("qwfpbjluy;arstgmneio'zxcdvkh,.."), false); // invalid
        assert_eq!(validate_base_layout_data("qwfpbjluy;arstgmneio'zxcdvk:,./"), true); // incomplete char
        assert_eq!(validate_base_layout_data(DEFAULT_LAYOUT_DATA), true);
    }

    #[test]
    fn test_calc_layout_difference() {
        let ws = Wordlist::new();

        assert_approx_eq!(calc_layout_difference(&ws,
            "flhvzqwuoysrntkcdeai;x'bmjpg,./",
            "flhvz'wuoysrntkcdeai;xjbmqpg,./"), 0.0024, 1e-4f64); // Semimak vs Semimak-JQ
        assert_approx_eq!(calc_layout_difference(&ws,
            "flhvz'wuoysrntkcdeai;xjbmqpg,./",
            "flhvz'wuoysrntkgdeai;xjbmqpc,./"), 0.0670, 1e-4f64); // Semimak-JQ vs Semimak-JQC
        assert_approx_eq!(calc_layout_difference(&ws,
            "qwertyuiopasdfghjkl;'zxcvbnm,./",
            "qwfpbjluy;arstgmneio'zxcdvkh,./"), 0.8355, 1e-4f64); // QWERTY vs Colemak-DH
        assert_approx_eq!(calc_layout_difference(&ws,
            "bldcvjyou,nrtsgphaei/xqmwzkf';.",
            "bldwz'foujnrtsgyhaei,qxmcvkp.-/"), 0.1478, 1e-4f64); // Gallium vs Graphite
    }
}