fn main() {
    // notedie::get_major_notes(6);
    let major_notes = notedie::major_scale(String::from("C"), notedie::Accidental::Natural);
    let natural_minor_notes =
        notedie::natural_minor_scale(String::from("A"), notedie::Accidental::Natural);
    println!("Major Scale: {:?}", major_notes);
    println!("Natural Minor Scale {:?}", natural_minor_notes);
}
