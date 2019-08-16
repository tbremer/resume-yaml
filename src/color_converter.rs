type RGB = (u16, u16, u16);
type HSL = (u16, u16, u16);

fn percent_change(ov: f32, change: f32) -> f32 {
    ov + ov * (change / 100.)
}

fn clamp(num: f32, max: f32) -> f32 {
    if num > max {
        max
    } else if num < 0. {
        0.
    } else {
        num
    }
}

#[derive(Debug)]
pub struct Colors {
    pub hex: String,
    pub rgb: RGB,
    pub hsl: HSL,
}

impl Colors {
    //color name | hue | saturation | luminace
    //---------------------------------
    // base      | 259 | 59         | 59
    //---------------------------------
    // dark      | 258 | 24         | 20
    //           |     | -60%       | -66%
    //---------------------------------
    // light     | 270 | 100        | 98
    //                 | 60%        | +66%
    //---------------------------------
    pub fn adjust_saturation(&self, change: i8) -> Colors {
        let (h, orignal_value, lum) = self.hsl;

        let new_value = clamp(percent_change(orignal_value as f32, change as f32), 100.).round();
        let hsl = (h, new_value as u16, lum);
        let rgb = hsl_to_rgb(hsl);

        Colors {
            hex: rgb_to_hex(rgb),
            rgb,
            hsl,
        }
    }

    pub fn adjust_luminance(&self, change: i8) -> Colors {
        let (h, s, orignal_value) = self.hsl;
        let new_value = clamp(percent_change(orignal_value as f32, change as f32), 100.).round();

        let hsl = (h, s, new_value as u16);
        let rgb = hsl_to_rgb(hsl);

        Colors {
            hex: rgb_to_hex(rgb),
            rgb,
            hsl,
        }
    }
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

fn rgb_to_hex((r,g, b): RGB) -> String {
    let mut r_hex = format!("{:X}", r);
    let mut g_hex = format!("{:X}", g);
    let mut b_hex = format!("{:X}", b);

    if r_hex.len() == 1 {
        if r < 16 {
            r_hex = format!("0{}", r_hex);
        }
        else {
            r_hex = format!("{}0", r_hex);
        };
    };

    if g_hex.len() == 1 {
        if g < 16 {
            g_hex = format!("0{}", g_hex);
        }
        else {
            g_hex = format!("{}0", g_hex);
        };
    };

    if b_hex.len() == 1 {
        if b < 16 {
            b_hex = format!("0{}", b_hex);
        }
        else {
            b_hex = format!("{}0", b_hex);
        };
    };

    format!("#{}{}{}",r_hex, g_hex, b_hex)
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

fn generate_temp_color(color: f32, lum_temp: f32, derived_temp: f32) -> f32 {
    if 6. * color < 1. {
        derived_temp + (lum_temp - derived_temp) * 6. * color
    }
    else if 2. * color < 1. {
        lum_temp
    }
    else if 3. * color < 2. {
        derived_temp + (lum_temp - derived_temp) * (0.666 - color) * 6.
    }
    else {
        derived_temp
    }
}

fn hsl_to_rgb((h,s,l): HSL) -> RGB {
    let hue = h as f32 / 360.;
    let sat = s as f32 / 100.;
    let lum = l as f32 / 100.;

    let lum_temp = if lum < 0.5 {
        lum * (1. + sat)
    }
    else {
        lum + sat - lum * sat
    };
    let derived_temp = 2. * lum - lum_temp;

    let r_channel = generate_temp_color(hue + 0.333, lum_temp, derived_temp);
    let g_channel = generate_temp_color(hue.clone(), lum_temp, derived_temp);
    let b_channel = generate_temp_color(hue - 0.333, lum_temp, derived_temp);

    (
        clamp((r_channel*255.).round(), 255.) as u16,
        clamp((g_channel*255.).round(), 255.) as u16,
        clamp((b_channel*255.).round(), 255.) as u16,
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
