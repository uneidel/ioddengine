use log::info;

pub fn round_by_displayformat(nbr : f64, displayformat : String)-> f64{
    if displayformat == "Dec" || displayformat == "Dec.0" {
        return nbr.round()
    }
    
    if displayformat.starts_with("Dec") {
        info!("DisplayFormat Raw: {}", displayformat);
        let parts: Vec<&str> = displayformat.split('.').collect();
        let decplaces = parts[1].parse::<usize>().unwrap();
        return round_to_precision(nbr, decplaces as u32);
    }
    nbr
}
fn round_to_precision(x: f64, decimals: u32) -> f64 {
    if x == 0. || decimals == 0 {
        return 0.0;
    }
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}

