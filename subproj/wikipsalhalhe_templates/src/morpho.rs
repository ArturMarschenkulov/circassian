use crate::ortho;
use crate::template;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum PreverbSoundForm {
    Full,        // e.g. къэ-
    Reduced,     // e.g. къы-
    BeforeVowel, // e.g. къ-
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transitivity {
    Transitive,
    Intransitive,
}
impl Transitivity {
    pub fn subject_case(&self) -> Case {
        match self {
            Transitivity::Transitive => Case::Ergative,
            Transitivity::Intransitive => Case::Absolutive,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preverb {
    base: String,
}

impl Preverb {
    pub fn new(base: &String) -> Self {
        Preverb {
            // form: PreverbSoundForm::Full,
            base: base.to_owned(),
        }
    }
    pub fn first_letter(&self) -> ortho::Letter {
        ortho::parse(&self.base)[0].clone()
    }
    pub fn last_consonant(&self) -> Option<ortho::Consonant> {
        let letters = ortho::parse(&self.base);
        let mut last_consonant = None;
        for letter in letters {
            match letter.kind {
                ortho::LetterKind::Consonant(consonant) => last_consonant = Some(consonant),
                _ => {}
            }
        }
        last_consonant
    }
    pub fn get_form(&self, form: &PreverbSoundForm) -> String {
        match &form {
            PreverbSoundForm::Full => self.base.clone(),
            PreverbSoundForm::Reduced => self.reduced(),
            PreverbSoundForm::BeforeVowel => self.before_vowel(),
        }
    }
    fn before_vowel(&self) -> String {
        let sss = match &self.base {
            base if base.ends_with('э') || base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();
                reduced
            }
            _ => unreachable!(),
        };
        sss
    }
    fn reduced(&self) -> String {
        let sss = match &self.base {
            base if base.ends_with('э') || base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();
                reduced + "ы"
            }
            _ => unreachable!(),
        };
        sss
    }

    fn get_string(&self, form: PreverbSoundForm) -> String {
        let sss = match &self.base {
            // This handles the preverbs which end on 'э'
            base if base.ends_with('э') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();

                match form {
                    PreverbSoundForm::Full => base.to_owned(),
                    PreverbSoundForm::Reduced => reduced + "ы",
                    PreverbSoundForm::BeforeVowel => reduced,
                }
            }
            base if base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();

                match form {
                    PreverbSoundForm::Full => base.to_owned(),
                    PreverbSoundForm::Reduced => reduced + "ы",
                    PreverbSoundForm::BeforeVowel => reduced,
                }
            }
            _ => unreachable!(),
        };
        sss
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MorphemeKind {
    Preverb(Preverb),
    PersonMarker(PersonMarker),
    NegationPrefix,
    RajImperative,

    Stem(template::VerbStem, String),

    Generic(String),
}

impl MorphemeKind {
    pub fn first_letter(&self) -> Option<ortho::Letter> {
        self.to_letters().first().cloned()
    }
    fn to_letters(&self) -> Vec<ortho::Letter> {
        let base = self.to_string();
        ortho::parse(&base)
    }
}

impl std::fmt::Display for MorphemeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MorphemeKind::Preverb(preverb) => write!(f, "{}", preverb.base),
            MorphemeKind::PersonMarker(person_marker) => {
                write!(f, "{}", person_marker.get_base_string())
            }
            MorphemeKind::NegationPrefix => write!(f, "мы"),
            MorphemeKind::RajImperative => write!(f, "ре"),
            MorphemeKind::Stem(stem, base) => write!(f, "{}", base),
            MorphemeKind::Generic(generic) => write!(f, "{}", generic),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Morpheme {
    pub kind: MorphemeKind,
    // base: String,
}
impl Morpheme {
    pub fn first_letter(&self) -> Option<ortho::Letter> {
        self.to_letters().first().cloned()
    }
    fn to_letters(&self) -> Vec<ortho::Letter> {
        let base = self.kind.to_string();
        ortho::parse(&base)
    }
}
impl std::fmt::Display for Morpheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl Morpheme {
    pub fn new_generic(base: &str) -> Self {
        Morpheme {
            kind: MorphemeKind::Generic(base.to_owned()),
            // base: base.to_owned(),
        }
    }
    pub fn new_negative_prefix() -> Self {
        Morpheme {
            kind: MorphemeKind::NegationPrefix,
            // base: "мы".to_owned(),
        }
    }
    pub fn new_imperative_raj() -> Self {
        Morpheme {
            kind: MorphemeKind::RajImperative,
            // base: "ре".to_owned(),
        }
    }
    pub fn new_preverb(preverb: &Preverb) -> Self {
        Morpheme {
            kind: MorphemeKind::Preverb(preverb.clone()),
            // base: preverb.base.clone(),
        }
    }
    pub fn new_person_marker(marker: &PersonMarker) -> Self {
        Morpheme {
            kind: MorphemeKind::PersonMarker(*marker),
            // base: marker.get_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Case {
    /// (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    /// (-м) subject of transitive verb
    Ergative,
    /// (-м) indirect object of intransitive and transitive verbs.
    Oblique,
}

/// A struct that indicates the number of a noun or verb.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Singular,
    Plural,
}
/// A struct that indicates the person of a verb.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PersonMarker {
    pub person: Person,
    pub number: Number,
    pub case: Case,
}

impl PersonMarker {
    pub fn new(person: Person, number: Number, case: Case) -> Self {
        PersonMarker {
            person,
            number,
            case,
        }
    }
}
impl PersonMarker {
    fn get_base_from(person: &Person, number: &Number, case: &Case) -> String {
        let pm = PersonMarker {
            person: *person,
            number: *number,
            case: *case,
        };
        pm.get_base_string()
    }
    pub fn get_base_string(&self) -> String {
        use Case::*;
        use Number::*;
        use Person::*;
        let result = match (self.person, self.number, self.case) {
            (First, Singular, Absolutive) => "сы",
            (First, Singular, Ergative) => "с",
            (First, Singular, Oblique) => "сэ",
            (Second, Singular, Absolutive) => "у",
            (Second, Singular, Ergative) => "б",
            (Second, Singular, Oblique) => "уэ",
            (Third, Singular, Absolutive) => "",
            (Third, Singular, Ergative) => "и",
            (Third, Singular, Oblique) => "е",
            (First, Plural, Absolutive) => "ды",
            (First, Plural, Ergative) => "д",
            (First, Plural, Oblique) => "дэ",
            (Second, Plural, Absolutive) => "фы",
            (Second, Plural, Ergative) => "ф",
            (Second, Plural, Oblique) => "фэ",
            (Third, Plural, Absolutive) => "",
            (Third, Plural, Ergative) => "я",
            (Third, Plural, Oblique) => "е",
        };

        result.to_string()
    }
    pub fn get_string(&self) -> String {
        Self::get_base_from(&self.person, &self.number, &self.case)
    }
}

pub fn new_masdar(
    polarity: &str,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root.clone()),
        //base: root.clone(),
    });
    morphemes.push_back(Morpheme::new_generic("н"));
    if polarity == "мы" {
        let m = Morpheme::new_negative_prefix();
        morphemes.push_front(m);
    }

    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }
    morphemes
}

pub fn new_masdar_personal(
    polarity: &str,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    assert_eq!(abs_marker.case, Case::Absolutive);

    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root.clone()),
        //base: root.clone(),
    });
    morphemes.push_back(Morpheme::new_generic("н"));

    // Add negative prefix
    if polarity == "мы" {
        let m = Morpheme::new_negative_prefix();
        morphemes.push_front(m);
    }

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        let marker = PersonMarker::new(marker.person, marker.number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    };

    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }

    // Add absolutive person marker
    if (Person::Third) != abs_marker.person {
        let marker = PersonMarker::new(abs_marker.person, abs_marker.number, Case::Absolutive);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }

    let m = Morpheme::new_person_marker(&abs_marker);
    morphemes.push_back(m);

    morphemes
}

pub fn new_imperative(
    polarity: &str,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root.clone()),
        // base: root.clone(),
    });

    // Add negative prefix
    if polarity == "мы" {
        let m = Morpheme::new_negative_prefix();
        morphemes.push_front(m);
    }

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        let marker = PersonMarker::new(Person::Second, marker.number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    };

    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }

    // Add absolutive person marker
    if (Person::Third) != abs_marker.person {
        let marker = PersonMarker::new(Person::Second, abs_marker.number, Case::Absolutive);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }

    morphemes
}
