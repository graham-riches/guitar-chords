#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Note {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B
}

impl Note {
    pub fn to_string(&self) -> String {
        match *self {
            Note::C  => "C ".to_string(),
            Note::Cs => "C#".to_string(),
            Note::D  => "D ".to_string(),
            Note::Ds => "D#".to_string(),
            Note::E  => "E ".to_string(),
            Note::F  => "F ".to_string(),
            Note::Fs => "F#".to_string(),
            Note::G  => "G ".to_string(),
            Note::Gs => "G#".to_string(),
            Note::A  => "A ".to_string(),
            Note::As => "A#".to_string(),
            Note::B  => "B ".to_string(),
        }
    }
}

impl IntoIterator for Note {
    type Item = Note;
    type IntoIter = NoteIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        NoteIntoIterator {
            note: self,
        }
    }
}

pub struct NoteIntoIterator {
    note: Note,
}

impl Iterator for NoteIntoIterator {
    type Item = Note;
    fn next(&mut self) -> Option<Note> {
        let n = match self.note {
            Note::C  => Note::Cs,
            Note::Cs => Note::D,
            Note::D  => Note::Ds,
            Note::Ds => Note::E,
            Note::E  => Note::F,
            Note::F  => Note::Fs,
            Note::Fs => Note::G,
            Note::G  => Note::Gs,
            Note::Gs => Note::A,
            Note::A  => Note::As,
            Note::As => Note::B,
            Note::B  => Note::C,
        };
        let temp = self.note;
        self.note = n;
        Some(temp)
    }
}