use super::math::round_by_displayformat;
use log::info;
pub fn pad_left(input: String, pad_char: char, total_width: usize) -> String {
    let padding = total_width.saturating_sub(input.len());
    let padded_string = pad_char.to_string().repeat(padding) + input.as_str();
    padded_string
}

pub fn convert_to_humanformat(
    gradient: Option<f32>,
    displayformat: Option<String>,
    offset: Option<u8>,
    mut total: f64,
) -> f64 {
    if let Some(g) = gradient {
        let g = g as f64;
        let offset = offset.unwrap();
        total = (total * g) + offset as f64;
        info!("Convert_to_humanformat-> Total: {}", total);
        let displayformat = displayformat.unwrap();
        total = round_by_displayformat(total, displayformat);
    }
    total
}


pub fn replace_special_character(c: &str) -> &str {
    match c {
        "<" => "&lt;",
        ">" => "&gt;",
        "\"" => "&quot;",
        "'" => "&apos;",
        "&" => "&amp;",
        _ => c,
    }
}
pub fn restore_special_character(c: &str) -> String {
    let mut result = String::new();
    let mut chars = c.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '&' {
            let mut entity = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == ';' {
                    chars.next(); // Consume ';'
                    break;
                } else {
                    entity.push(next_ch);
                    chars.next(); // Consume the character
                }
            }

            match &entity[..] {
                "lt" => result.push('<'),
                "gt" => result.push('>'),
                "quot" => result.push('"'),
                "apos" => result.push('\''),
                "amp" => result.push('&'),
                _ => {
                    result.push('&');
                    result.push_str(&entity);
                    result.push(';');
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}
