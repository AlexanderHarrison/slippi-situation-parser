fn main() {
    for (_, game_info) in slp_parser::read_info_in_dir("/home/alex/Slippi/").unwrap() {
        println!("{}", game_info.stage);
    }
}
