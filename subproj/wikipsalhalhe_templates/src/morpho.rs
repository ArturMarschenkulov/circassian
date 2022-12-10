use crate::ortho;
use crate::wiki::template;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn pronoun_row(&self) -> [&str; 6] {
        match &self {
            Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
            Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polarity {
    Positive,
    Negative,
}
impl Polarity {
    pub fn to_string_prefix(&self) -> String {
        match self {
            Polarity::Positive => "".to_owned(),
            Polarity::Negative => "мы".to_owned(),
        }
    }
    pub fn to_string_suffix(&self) -> String {
        match self {
            Polarity::Positive => "".to_owned(),
            Polarity::Negative => "къым".to_owned(),
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
        ortho::parse(&self.to_string())
    }
}

impl std::fmt::Display for MorphemeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MorphemeKind::Preverb(preverb) => write!(f, "{}", preverb.base),
            MorphemeKind::PersonMarker(person_marker) => {
                write!(f, "{}", person_marker.base_string())
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
        }
    }
    pub fn new_negative_prefix() -> Self {
        Morpheme {
            kind: MorphemeKind::NegationPrefix,
        }
    }
    pub fn new_imperative_raj() -> Self {
        Morpheme {
            kind: MorphemeKind::RajImperative,
        }
    }
    pub fn new_preverb(preverb: &Preverb) -> Self {
        Morpheme {
            kind: MorphemeKind::Preverb(preverb.clone()),
        }
    }
    pub fn new_person_marker(marker: &PersonMarker) -> Self {
        Morpheme {
            kind: MorphemeKind::PersonMarker(*marker),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    /// (-м) subject of transitive verb
    Ergative,
    /// (-м) indirect object of intransitive and transitive verbs.
    Oblique,
}

/// A struct that indicates the number of a noun or verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    Singular,
    Plural,
}
/// A struct that indicates the person of a verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersonMarker {
    pub person: Person,
    pub number: Number,
    /// The case of the person marker.
    /// However, not direct reflection of the actual case, but more so the "surface level" appearance of the person markers.
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

    pub fn to_letters(&self) -> Vec<ortho::Letter> {
        let base = self.base_string();
        ortho::parse(&base)
    }
    pub fn is_second_singular(&self) -> bool {
        self.person == Person::Second && self.number == Number::Singular
    }
    pub fn is_third_singular_ergative(&self) -> bool {
        self.person == Person::Third
            && self.number == Number::Singular
            && self.case == Case::Ergative
    }
    pub fn is_third_plural_ergative(&self) -> bool {
        self.person == Person::Third && self.number == Number::Plural && self.case == Case::Ergative
    }
    pub fn is_ergative(&self) -> bool {
        self.case == Case::Ergative
    }
    pub fn is_absolutive(&self) -> bool {
        self.case == Case::Absolutive
    }
}
impl PersonMarker {
    pub fn base_string_as_voiced(&self) -> String {
        let base = self.base_string();
        let old = base.chars().next().unwrap();
        let new = match old {
            'с' => 'з',
            'ф' => 'в',
            x => x,
        };
        base.replacen(old, &new.to_string(), 1)
    }
    pub fn base_string_as_voiceless(&self) -> String {
        let base = self.base_string();
        let old = base.chars().next().unwrap();
        let new = match old {
            'б' => 'п',
            'д' => 'т',
            x => x,
        };
        base.replacen(old, &new.to_string(), 1)
    }
    pub fn base_string_as_after_consonant(&self) -> String {
        let base = self.base_string();
        let old = base.chars().next().unwrap();
        let new = match old {
            'я' => 'а',
            x => x,
        };
        base.replacen(old, &new.to_string(), 1)
    }
    pub fn base_string_as_before_o(&self) -> String {
        let base = self.base_string();
        if base.ends_with('ы') {
            base.replacen('ы', "", 1)
        } else {
            base
        }
    }
    pub fn base_string(&self) -> String {
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
}

pub fn new_masdar(
    polarity: &Polarity,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
        //base: root,
    });
    morphemes.push_back(Morpheme::new_generic("н"));

    // Prefix part

    if polarity == &Polarity::Negative {
        let m = Morpheme::new_negative_prefix();
        morphemes.push_front(m);
    }

    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }
    morphemes
}

pub fn new_imperative_raj(
    polarity: &Polarity,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    person: &Person,
    number: &Number,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
    // Add stem
    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
        // base: root,
    });

    // Prefix part

    // Add negative prefix
    if polarity == &Polarity::Negative {
        let m = Morpheme::new_negative_prefix();
        morphemes.push_front(m);
    }
    // Add imperative raj
    morphemes.push_front(Morpheme::new_imperative_raj());
    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }

    // Add person marker
    // If there is a preverb present, the third person marker is not present
    if !(preverb.is_some() && Person::Third == *person) {
        let number = if (person, number) == (&Person::Third, &Number::Plural) {
            Number::Singular
        } else {
            *number
        };
        let marker = PersonMarker::new(*person, number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }
    morphemes
}
pub fn new_masdar_personal(
    polarity: &Polarity,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    assert_eq!(abs_marker.case, Case::Absolutive);

    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
        //base: root,
    });
    // Suffix part

    morphemes.push_back(Morpheme::new_generic("н"));

    // Prefix part

    // Add negative prefix
    if polarity == &Polarity::Negative {
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

    // let m = Morpheme::new_person_marker(&abs_marker);
    // morphemes.push_back(m);

    morphemes
}

pub fn new_imperative(
    polarity: &Polarity,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
        // base: root,
    });

    // Prefix part

    // Add negative prefix
    if polarity == &Polarity::Negative {
        let m = Morpheme::new_negative_prefix();
        morphemes.push_front(m);
    }

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        if (polarity, marker.person, marker.number)
            != (&Polarity::Negative, Person::Second, Number::Singular)
        {
            let marker = PersonMarker::new(Person::Second, marker.number, Case::Ergative);
            let m = Morpheme::new_person_marker(&marker);
            morphemes.push_front(m);
        }
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
