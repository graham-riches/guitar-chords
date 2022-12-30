use crate::guitar::{GuitarBuilder};
use crate::music::Note;

pub mod guitar;
pub mod music;

fn main() {
    let mut guitar = GuitarBuilder::new()
     .with_frets(20)
     .with_tuning(vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E])
     .build();

    guitar.set_interval(Note::C, 1);
    guitar.set_interval(Note::E, 3);
    guitar.set_interval(Note::B, 7);
    println!("{:?}", guitar);
}
