pub fn get_excerpts() -> Vec<String> {
    let black_holes = include_str!("./excerpts/black_holes.txt").to_string();
    let harry_potter = include_str!("./excerpts/harry_potter_1.txt").to_string();
    let star_wars = include_str!("./excerpts/star_wars_1.txt").to_string();
    let theory = include_str!("./excerpts/theory_relativity.txt").to_string();

    vec![black_holes, harry_potter, star_wars, theory]
}