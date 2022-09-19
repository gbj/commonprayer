fn main() {
    for psalm_number in 1..=150 {
        println!("building JSON for psalm {psalm_number}");

        let bcp_file = std::fs::File::create(&format!(
            "./static/psalter/bcp1979/psalm-{:03}.json",
            psalm_number
        ))
        .expect("could not open psalm file");
        let bcp = psalter::bcp1979::BCP1979_PSALTER
            .psalm_by_number(psalm_number)
            .unwrap();
        serde_json::to_writer(bcp_file, bcp);

        let loc_file = std::fs::File::create(&format!(
            "./static/psalter/loc/psalm-{:03}.json",
            psalm_number
        ))
        .expect("could not open psalm file");
        let loc = psalter::loc::LOC_PSALTER
            .psalm_by_number(psalm_number)
            .unwrap();
        serde_json::to_writer(loc_file, loc);
    }
}
