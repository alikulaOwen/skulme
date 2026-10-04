/// Author : https://github.com/ali77gh\
/// References:\
/// RGB:  https://en.wikipedia.org/wiki/RGB_color_model\
/// CMYK: https://en.wikipedia.org/wiki/CMYK_color_model\

/// This function Converts RGB to CMYK format
///
/// ### Params
/// * `r` - red
/// * `g` - green
/// * `b` - blue
///
/// ### Returns
/// (C, M, Y, K)
pub fn rgb_to_cmyk(rgb: (u8, u8, u8)) -> (u8, u8, u8, u8) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rgb_to_cmyk)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rgb_to_cmyk");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_rgb_to_cmyk {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (rgb, cmyk) = $tc;
                    assert_eq!(rgb_to_cmyk(rgb), cmyk);
                }
            )*
        }
    }

    test_rgb_to_cmyk! {
        white: ((255, 255, 255), (0, 0, 0, 0)),
        gray: ((128, 128, 128), (0, 0, 0, 49)),
        black: ((0, 0, 0), (0, 0, 0, 100)),
        red: ((255, 0, 0), (0, 100, 100, 0)),
        green: ((0, 255, 0), (100, 0, 100, 0)),
        blue: ((0, 0, 255), (100, 100, 0, 0)),
    }
}
