fn main() {
    // notedie::get_major_notes(6);
    let major_notes = notedie::major_scale("C", notedie::Accidental::Natural);
    let natural_minor_notes = notedie::natural_minor_scale("A", notedie::Accidental::Natural);
    let maj7_notes = notedie::major7("C");

    println!("Major Scale: {:?}", major_notes);
    println!("Natural Minor Scale {:?}", natural_minor_notes);
    println!("Major 7 Arpeggio {:?}", maj7_notes);
}
