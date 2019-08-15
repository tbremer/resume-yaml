type RGB = (u16, u16, u16);
type HSL = (u16, u16, u16);

#[derive(Debug)]
pub struct Colors {
    pub hex: String,
    pub rgb: RGB,
    pub hsl: HSL,
}

fn hex_to_decimal(hex_pair: &str) -> u16 {
    match u16::from_str_radix(hex_pair, 16) {
        Ok(v) => v,
        Err(_) => {
            println!("Unprocessable Hex value: {}", hex_pair);
            std::process::exit(128);
        }
    }
}

fn hex_to_rgb(hex: &str) -> RGB {
    if "#" == &hex[..1] {
        println!("Please discard `#` at start of string");
        std::process::exit(128);
    }

    let hex_string = match hex.len() {
        3 => format!(
            "{}{}{}{}{}{}",
            &hex[..1],
            &hex[..1],
            &hex[1..2],
            &hex[1..2],
            &hex[2..3],
            &hex[2..3]
        ),
        6 => hex.to_string(),
        l => {
            println!("{} has length of {}, expected 3 or 6", hex, l);
            std::process::exit(128)
        }
    };

    (
        hex_to_decimal(&hex_string[0..2]),
        hex_to_decimal(&hex_string[2..4]),
        hex_to_decimal(&hex_string[4..]),
    )
}

fn rgb_to_hsl((r, g, b): RGB) -> HSL {
    use std::f32::{MAX, MIN};

    // cast as floats
    // math the floats
    let r = (r as f32) / 255.;
    let g = (g as f32) / 255.;
    let b = (b as f32) / 255.;
    let range = vec![r, g, b];

    // get min and max
    let min = range.clone().iter().fold(MAX, |a, &b| a.min(b));
    let max = range.clone().iter().fold(MIN, |a, &b| a.max(b));

    // calc luminace
    let lum = (min + max) / 2.;

    // calc saturation
    let saturation = if lum < 0.5 {
        (max - min) / (max + min)
    } else {
        (max - min) / (2. - max - min)
    };

    let hue = if max == r {
        ((g - b) / (max - min)) * 60.
    } else if max == g {
        (2. + (b - r) / (max - min)) * 60.
    } else {
        (4. + (r - g) / (max - min)) * 60.
    };

    let hue = if hue < 0. { hue + 360. } else { hue };

    (
        hue.round() as u16,
        ((saturation * 100.).round()) as u16,
        ((lum * 100.).round()) as u16,
    )
}

pub fn hex_to_hsl(hex: &str) -> Colors {
    let hex = if "#" == &hex[..1] { &hex[1..] } else { hex };

    let rgb = hex_to_rgb(hex);
    let hsl = rgb_to_hsl(rgb);

    Colors {
        hex: format!("#{}", hex),
        rgb,
        hsl,
    }
}
