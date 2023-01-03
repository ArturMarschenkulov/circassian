use crate::ortho;
use crate::wiki::template;

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tense {
    Present,
    Imperfect,
    Perfect,
    PlusQuamPerfect,
    FarPast1,
    FarPast2,
    Aorist1,
    Aorist2,
    Future1,
    Future2,
}
impl Tense {
    pub fn variants() -> Vec<Self> {
        [
            Tense::Present,
            Tense::Imperfect,
            Tense::Perfect,
            Tense::PlusQuamPerfect,
            Tense::FarPast1,
            Tense::FarPast2,
            Tense::Aorist1,
            Tense::Aorist2,
            Tense::Future1,
            Tense::Future2,
        ]
        .to_vec()
    }
    pub fn variants_iter() -> impl Iterator<Item = Self> {
        Self::variants().into_iter()
    }
}

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
}

pub struct Pronoun {
    pub case: Case,
    pub number: Number,
    pub person: Person,
}

impl Pronoun {
    pub fn variants_case(case: &Case) -> Vec<Self> {
        Number::variants_iter()
            .flat_map(|n| {
                Person::variants_iter().map(move |p| Pronoun {
                    case: *case,
                    number: n,
                    person: p,
                })
            })
            .collect::<Vec<_>>()
    }
    pub fn variants_person(person: &Person) -> Vec<Self> {
        Number::variants_iter()
            .flat_map(|n| {
                Case::variants_iter().map(move |c| Pronoun {
                    case: c,
                    number: n,
                    person: *person,
                })
            })
            .collect::<Vec<_>>()
    }
}

impl std::fmt::Display for Pronoun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self.person, self.number, self.case) {
                (Person::First, Number::Singular, _) => "сэ",
                (Person::Second, Number::Singular, _) => "уэ",
                (Person::First, Number::Plural, _) => "дэ",
                (Person::Second, Number::Plural, _) => "фэ",

                (Person::Third, Number::Singular, Case::Absolutive) => "ар",
                (Person::Third, Number::Plural, Case::Absolutive) => "ахэр",
                (Person::Third, Number::Singular, Case::Ergative) => "абы",
                (Person::Third, Number::Plural, Case::Ergative) => "абыхэм",
            }
            .to_owned()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    Positive,
    Negative,
}
impl Polarity {
    pub fn variants() -> Vec<Self> {
        [Polarity::Positive, Polarity::Negative].to_vec()
    }
    pub fn variants_iter() -> impl Iterator<Item = Self> {
        Self::variants().into_iter()
    }
    pub fn to_string_prefix(self) -> String {
        match self {
            Polarity::Positive => "".to_owned(),
            Polarity::Negative => "мы".to_owned(),
        }
    }
    pub fn to_string_suffix(self) -> String {
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
            if let ortho::LetterKind::Consonant(consonant) = letter.kind {
                last_consonant = Some(consonant)
            }
        }
        last_consonant
    }
    pub fn form(&self, form: &PreverbSoundForm) -> String {
        match &form {
            PreverbSoundForm::Full => self.base.clone(),
            PreverbSoundForm::Reduced => self.reduced(),
            PreverbSoundForm::BeforeVowel => self.before_vowel(),
        }
    }
    fn before_vowel(&self) -> String {
        match &self.base {
            base if base.ends_with('э') || base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();
                reduced
            }
            _ => unreachable!(),
        }
    }
    fn reduced(&self) -> String {
        match &self.base {
            base if base.ends_with('э') || base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();
                reduced + "ы"
            }
            _ => unreachable!(),
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn last_latter(&self) -> Option<ortho::Letter> {
        self.to_letters().last().cloned()
    }
    fn to_letters(&self) -> Vec<ortho::Letter> {
        ortho::parse(&self.to_string())
    }

    pub fn is_generic_certain(&self, generic: &str) -> bool {
        match self {
            MorphemeKind::Generic(g) => g == generic,
            _ => false,
        }
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
            MorphemeKind::Stem(_, base) => write!(f, "{}", base),
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
    pub fn to_letters(&self) -> Vec<ortho::Letter> {
        let base = self.kind.to_string();
        ortho::parse(&base)
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

impl std::fmt::Display for Morpheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    /// (-м) subject of transitive verb
    Ergative,
}
impl Case {
    pub fn variants() -> Vec<Self> {
        vec![Case::Absolutive, Case::Ergative]
    }
    pub fn variants_iter() -> impl Iterator<Item = Self> {
        Self::variants().into_iter()
    }
}

/// A struct that indicates the number of a noun or verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    Singular,
    Plural,
}
impl Number {
    pub fn variants() -> Vec<Self> {
        vec![Number::Singular, Number::Plural]
    }
    pub fn variants_iter() -> impl Iterator<Item = Self> {
        Self::variants().into_iter()
    }
}

/// A struct that indicates the person of a verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Person {
    First,
    Second,
    Third,
}
impl Person {
    pub fn variants() -> Vec<Self> {
        vec![Person::First, Person::Second, Person::Third]
    }
    pub fn variants_iter() -> impl Iterator<Item = Self> {
        Self::variants().into_iter()
    }
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

    pub fn to_letters(self) -> Vec<ortho::Letter> {
        ortho::parse(&self.base_string())
    }
}

impl PersonMarker {
    pub fn is_third(&self) -> bool {
        self.person == Person::Third
    }
    pub fn is_second_singular(&self) -> bool {
        self.person == Person::Second && self.number == Number::Singular
    }

    pub fn is_third_ergative(&self) -> bool {
        self.person == Person::Third && self.case == Case::Ergative
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
            x if ['б', 'д', 'и'].contains(&x) => x,
            x => unreachable!("Unexpected letter: {}", x),
        };
        base.replacen(old, &new.to_string(), 1)
    }

    pub fn base_string_as_voiceless(&self) -> String {
        let base = self.base_string();
        let old = base.chars().next().unwrap();
        let new = match old {
            'б' => 'п',
            'д' => 'т',
            x if ['с', 'ф', 'и'].contains(&x) => x,
            x => unreachable!("Unexpected letter: {}", x),
        };
        base.replacen(old, &new.to_string(), 1)
    }
    pub fn base_string_as_after_consonant(&self) -> String {
        let base = self.base_string();
        let old = base.chars().next().unwrap();
        let new = match old {
            'я' => 'а',
            x if ['с', 'б', 'д', 'ф', 'и'].contains(&x) => x,
            x => unreachable!("Unexpected letter: {}", x),
        };
        base.replacen(old, &new.to_string(), 1)
    }

    // pub fn base_string_as_before_m(&self) -> String {
    //     let base = self.base_string();
    // }
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
            (Second, Singular, Absolutive) => "у",
            (Third, Singular, Absolutive) => "",
            (First, Plural, Absolutive) => "ды",
            (Second, Plural, Absolutive) => "фы",
            (Third, Plural, Absolutive) => "",

            (First, Singular, Ergative) => "с",
            (Second, Singular, Ergative) => "б",
            (Third, Singular, Ergative) => "и",
            (First, Plural, Ergative) => "д",
            (Second, Plural, Ergative) => "ф",
            (Third, Plural, Ergative) => "я",
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
    let marker = PersonMarker::new(abs_marker.person, abs_marker.number, Case::Absolutive);
    let m = Morpheme::new_person_marker(&marker);
    morphemes.push_front(m);

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
    let marker = PersonMarker::new(Person::Second, abs_marker.number, Case::Absolutive);
    let m = Morpheme::new_person_marker(&marker);
    morphemes.push_front(m);

    morphemes
}

pub fn new_indicative(
    polarity: &Polarity,
    tense: &Tense,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
    let tense_suffix_pair = match tense {
        Tense::Present => ("р", "ркъым"),
        Tense::Imperfect => ("рт", "ртэкъым"),
        Tense::Perfect => ("ащ", "акъым"),
        Tense::PlusQuamPerfect => ("ат", "атэкъым"),
        Tense::FarPast1 => ("гъащ", "гъакъым"),
        Tense::FarPast2 => ("гъат", "гъатэкъым"),
        Tense::Aorist1 => ("щ", "къым"),
        Tense::Aorist2 => ("т", "тэкъым"),
        Tense::Future1 => ("нщ", "нкъым"),
        Tense::Future2 => ("ну", "нукъым"),
    };
    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
    });
    morphemes.push_back(Morpheme::new_generic(if polarity == &Polarity::Positive {
        tense_suffix_pair.0
    } else {
        tense_suffix_pair.1
    }));

    if (polarity, tense) == (&Polarity::Positive, &Tense::Present) {
        morphemes.push_front(Morpheme::new_generic("о"));
    }

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        let marker = PersonMarker::new(marker.person, marker.number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }

    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }

    // Add absolutive person marker
    let marker = PersonMarker::new(abs_marker.person, abs_marker.number, Case::Absolutive);
    let m = Morpheme::new_person_marker(&marker);
    morphemes.push_front(m);

    morphemes
}

pub fn new_interrogative(
    polarity: &Polarity,
    tense: &Tense,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
    let tense_suffix_pair = match tense {
        Tense::Present => ("рэ", "ркъэ"),
        Tense::Imperfect => ("рт", "ртэкъым"),
        Tense::Perfect => ("ащ", "акъэ"),
        Tense::PlusQuamPerfect => ("ат", "атэкъэ"),
        Tense::FarPast1 => ("гъащ", "гъакъэ"),
        Tense::FarPast2 => ("гъат", "гъатэкъэ"),
        Tense::Aorist1 => ("", "къэ"),
        Tense::Aorist2 => ("т", "тэкъэ"),
        Tense::Future1 => ("нщ", "нкъэ"),
        Tense::Future2 => ("ну", "нукъэ"),
    };
    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
    });

    morphemes.push_back(Morpheme::new_generic(if polarity == &Polarity::Positive {
        tense_suffix_pair.0
    } else {
        tense_suffix_pair.1
    }));

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        let marker = PersonMarker::new(marker.person, marker.number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }

    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }

    // Add absolutive person marker
    let marker = PersonMarker::new(abs_marker.person, abs_marker.number, Case::Absolutive);
    let m = Morpheme::new_person_marker(&marker);
    morphemes.push_front(m);

    morphemes
}

pub fn new_conditional(
    polarity: &Polarity,
    tense: &Tense,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
    let tense_suffix = match tense {
        Tense::Present => "",
        Tense::Perfect => "а",
        Tense::FarPast1 => "гъа",
        Tense::Future1 => "н",
        Tense::Future2 => "ну",
        _ => unreachable!("Invalid tense for conditional: {:?}", tense),
    };
    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
    });
    if !tense_suffix.is_empty() {
        morphemes.push_back(Morpheme::new_generic(tense_suffix));
    }

    morphemes.push_back(Morpheme::new_generic("мэ"));

    // negation prefix
    if polarity == &Polarity::Negative {
        morphemes.push_front(Morpheme::new_negative_prefix());
    }

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        let marker = PersonMarker::new(marker.person, marker.number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }
    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }
    // Add absolutive person marker
    let marker = PersonMarker::new(abs_marker.person, abs_marker.number, Case::Absolutive);
    let m = Morpheme::new_person_marker(&marker);
    morphemes.push_front(m);

    morphemes
}

pub fn new_conditional_2(
    polarity: &Polarity,
    tense: &Tense,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
    let tense_suffix = match tense {
        Tense::Present => "тэ",
        Tense::Perfect => "атэ",
        Tense::FarPast1 => "гъатэ",
        Tense::Future1 => "нтэ",
        Tense::Future2 => "нутэ",
        _ => unreachable!("Invalid tense for conditional: {:?}", tense),
    };
    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
    });

    if !tense_suffix.is_empty() {
        morphemes.push_back(Morpheme::new_generic(tense_suffix));
    }

    morphemes.push_back(Morpheme::new_generic("мэ"));

    // negation prefix
    if polarity == &Polarity::Negative {
        morphemes.push_front(Morpheme::new_negative_prefix());
    }

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        let marker = PersonMarker::new(marker.person, marker.number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }
    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }

    let marker = PersonMarker::new(abs_marker.person, abs_marker.number, Case::Absolutive);
    let m = Morpheme::new_person_marker(&marker);
    morphemes.push_front(m);

    morphemes
}

pub fn new_subjunctive(
    polarity: &Polarity,
    tense: &Tense,
    preverb: &Option<Preverb>,
    stem: &template::VerbStem,
    abs_marker: &PersonMarker,
    erg_marker: &Option<PersonMarker>,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
    let tense_suffix_pair = match tense {
        Tense::Future1 => ("нт", "нтэкъым"),
        Tense::Future2 => ("нут", "нутэкъым"),
        _ => unreachable!("Invalid tense for conditional: {:?}", tense),
    };
    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root),
    });

    morphemes.push_back(Morpheme::new_generic(if polarity == &Polarity::Positive {
        tense_suffix_pair.0
    } else {
        tense_suffix_pair.1
    }));

    // Add ergative person marker
    if let Some(marker) = erg_marker {
        let marker = PersonMarker::new(marker.person, marker.number, Case::Ergative);
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }

    // Add preverb
    if let Some(preverb) = preverb.clone() {
        let m = Morpheme::new_preverb(&preverb);
        morphemes.push_front(m);
    }

    // Add absolutive person marker
    let marker = PersonMarker::new(abs_marker.person, abs_marker.number, Case::Absolutive);
    let m = Morpheme::new_person_marker(&marker);
    morphemes.push_front(m);

    morphemes
}
