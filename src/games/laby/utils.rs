pub fn display_bar(width: usize, value: f64, total_value: f64) -> String {
    let percent = value / total_value;
    let nb_full_tiles = (percent * (width - 2) as f64).ceil() as usize;
    let rest_tiles = width - 2 - nb_full_tiles;

    let mut res = String::from("[");
    res.push_str(&String::from("=").repeat(nb_full_tiles));
    res.push_str(&String::from(" ").repeat(rest_tiles));
    res.push_str(&format!("] {:.2}/{:.2}", value, total_value));

    res
}
