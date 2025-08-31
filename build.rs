use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");

    let content = fs::read_to_string("src/main.rs").expect("failed to read main.rs");
    let marker = "__START_DATA__";
    let marker_pos = content.find(marker).expect("marker not found") + marker.len();
    let data_part = &content[marker_pos..];
    let joined: String = data_part.lines().map(|line| line.trim()).collect();
    let chars: String = joined.chars().map(|c| char::from_u32(c as u32 + 200).unwrap()).collect();

    let output = format!("pub const DATA: &str = {:?};", chars);
    fs::write("src/data.rs", output).expect("failed to write data.rs");
}
