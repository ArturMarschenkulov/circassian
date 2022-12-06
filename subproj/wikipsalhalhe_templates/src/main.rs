// #![allow(dead_code, unused_variables)]
// #![allow(clippy::match_like_matches_macro)]

mod evaluation;
mod ortho;
mod table;
mod template;

use std::collections::VecDeque;

use table::Wikitable;
use template::VerbStem;

#[derive(Debug, Clone, PartialEq)]
enum PreverbSoundForm {
    Full,        // e.g. къэ-
    Reduced,     // e.g. къы-
    BeforeVowel, // e.g. къ-
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preverb {
    base: String,
}
impl Preverb {
    fn new(base: &String) -> Self {
        Preverb {
            // form: PreverbSoundForm::Full,
            base: base.to_owned(),
        }
    }
    fn first_letter(&self) -> ortho::Letter {
        ortho::parse(&self.base)[0].clone()
    }
    fn last_consonant(&self) -> Option<ortho::Consonant> {
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
    fn get_form(&self, form: &PreverbSoundForm) -> String {
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
enum MorphemeKind {
    Preverb(Preverb),
    PersonMarker(PersonMarker),
    NegationPrefix,
    RajImperative,

    Stem(template::VerbStem, String),

    Generic(String),
}

impl MorphemeKind {
    fn first_letter(&self) -> Option<ortho::Letter> {
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
    kind: MorphemeKind,
    // base: String,
}
impl Morpheme {
    fn first_letter(&self) -> Option<ortho::Letter> {
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
    fn new_generic(base: &str) -> Self {
        Morpheme {
            kind: MorphemeKind::Generic(base.to_owned()),
            // base: base.to_owned(),
        }
    }
    fn new_negative_prefix() -> Self {
        Morpheme {
            kind: MorphemeKind::NegationPrefix,
            // base: "мы".to_owned(),
        }
    }
    fn new_imperative_raj() -> Self {
        Morpheme {
            kind: MorphemeKind::RajImperative,
            // base: "ре".to_owned(),
        }
    }
    fn new_preverb(preverb: &Preverb) -> Self {
        Morpheme {
            kind: MorphemeKind::Preverb(preverb.clone()),
            // base: preverb.base.clone(),
        }
    }
    fn new_person_marker(marker: &PersonMarker) -> Self {
        Morpheme {
            kind: MorphemeKind::PersonMarker(*marker),
            // base: marker.get_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Case {
    /// (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    /// (-м) subject of transitive verb
    Ergative,
    /// (-м) indirect object of intransitive and transitive verbs.
    Oblique,
}

/// A struct that indicates the number of a noun or verb.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Number {
    Singular,
    Plural,
}
/// A struct that indicates the person of a verb.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PersonMarker {
    person: Person,
    number: Number,
    case: Case,
}

impl PersonMarker {
    fn new(person: Person, number: Number, case: Case) -> Self {
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
    fn get_base_string(&self) -> String {
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
    fn get_string(&self) -> String {
        Self::get_base_from(&self.person, &self.number, &self.case)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VowelKind {
    With,
    Without,
    Alternating,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transitivity {
    Transitive,
    Intransitive,
}
impl Transitivity {
    fn subject_case(&self) -> Case {
        match self {
            Transitivity::Transitive => Case::Ergative,
            Transitivity::Intransitive => Case::Absolutive,
        }
    }
}

/*
    {| class="wikitable"
    |-
    ! Инфинитив (масдар) !!
    |-
    | щыӀэныгъэ: || {{{псалъэпкъ}}}эн
    |-
    | щымыӀэныгъэ: || мы{{{псалъэпкъ}}}эн
    |}
*/

fn new_masdar(polarity: &str, preverb: &Option<Preverb>, stem: &VerbStem) -> VecDeque<Morpheme> {
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

fn table_masdar(desc: &template::TemplateDesc) -> String {
    // let root = "{{{псалъэпкъ}}}".to_owned();
    let table_name = "Инфинитив (масдар)".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    table.add("".to_owned());

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ:", polarity));

        let morphemes = new_masdar(polarity, &desc.preverb, &desc.stem);
        let string = evaluation::evaluate_morphemes(&morphemes);
        table.add(string);
    }

    table.to_string()
}

/*
    {| class="wikitable"
    |-
    ! Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа !! сэ  !! уэ !! ар !! дэ !! фэ !! ахэр
    |-
    | щыӀэныгъэ: || сы{{{псалъэпкъ}}}эн || у{{{псалъэпкъ}}}эн || {{{псалъэпкъ}}}эн || ды{{{псалъэпкъ}}}эн || фы{{{псалъэпкъ}}}эн || {{{псалъэпкъ}}}эн(хэ)
    |-
    | щымыӀэныгъэ: || сымы{{{псалъэпкъ}}}эн || умы{{{псалъэпкъ}}}эн || мы{{{псалъэпкъ}}}эн || дымы{{{псалъэпкъ}}}эн || фымы{{{псалъэпкъ}}}эн || мы{{{псалъэпкъ}}}эн(хэ)
    |}
*/
fn new_masdar_personal(
    polarity: &str,
    preverb: &Option<Preverb>,
    stem: &VerbStem,
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
fn table_masdar_personal(desc: &template::TemplateDesc) -> String {
    let table_name = "Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    let subject_case = &desc.transitivity.subject_case();
    let pronouns = match &desc.transitivity {
        Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
        Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
    };

    for pronoun in pronouns.iter() {
        table.add(pronoun.to_string());
    }
    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));

        for number in &[Number::Singular, Number::Plural] {
            for person in &[Person::First, Person::Second, Person::Third] {
                let abs_marker = if subject_case == &Case::Absolutive {
                    PersonMarker::new(*person, *number, Case::Absolutive)
                } else {
                    PersonMarker::new(Person::Third, *number, Case::Absolutive)
                };
                let erg_marker = if subject_case == &Case::Ergative {
                    Some(PersonMarker::new(*person, *number, Case::Ergative))
                } else {
                    None
                };
                let morphemes = new_masdar_personal(
                    polarity,
                    &desc.preverb,
                    &desc.stem,
                    &abs_marker,
                    &erg_marker,
                );

                let string = evaluation::evaluate_morphemes(&morphemes);
                // println!("{:?}", evaluation::morphemes_to_string(&morphemes));
                table.add(string);
            }
        }
    }
    table.to_string()
}

/*
{| class="wikitable"
|-
! унафэ наклоненэ !! уэ !! фэ
|-
| щыӀэныгъэ: || {{{псалъэпкъ}}}э! || фы{{{псалъэпкъ}}}э!
|-
| щымыӀэныгъэ: || умы{{{псалъэпкъ}}}э! || фымы{{{псалъэпкъ}}}э!
|}
*/
fn new_imperative(
    polarity: &str,
    preverb: &Option<Preverb>,
    stem: &VerbStem,
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
fn table_imperative(desc: &template::TemplateDesc) -> String {
    let table_name = "унафэ наклоненэ".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    for pronoun in ["уэ", "фэ"].iter() {
        table.add(pronoun.to_string());
    }

    let subject_case = &desc.transitivity.subject_case();

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            let abs_marker = if subject_case == &Case::Absolutive {
                PersonMarker::new(Person::Second, *number, Case::Absolutive)
            } else {
                PersonMarker::new(Person::Third, *number, Case::Absolutive)
            };
            let erg_marker = if subject_case == &Case::Ergative {
                Some(PersonMarker::new(Person::Second, *number, Case::Ergative))
            } else {
                None
            };

            let morphemes = new_imperative(
                polarity,
                &desc.preverb,
                &desc.stem,
                &abs_marker,
                &erg_marker,
            );

            let string = evaluation::evaluate_morphemes(&morphemes);
            table.add(string);
        }
    }
    table.to_string()
}

/*
{| class="wikitable"
|-
! Ре-кӀэ унафэ наклоненэ !! сэ  !! уэ !! ар !! дэ !! фэ !! ахэр
|-
| щыӀэныгъэ: || сре{{{псалъэпкъ}}}э || уре{{{псалъэпкъ}}}э || ире{{{псалъэпкъ}}}э || дре{{{псалъэпкъ}}}э || фре{{{псалъэпкъ}}}э || ире{{{псалъэпкъ}}}э
|-
| щымыӀэныгъэ: || сремы{{{псалъэпкъ}}}э || уремы{{{псалъэпкъ}}}э || иремы{{{псалъэпкъ}}}э || дремы{{{псалъэпкъ}}}э || фремы{{{псалъэпкъ}}}э || иремы{{{псалъэпкъ}}}э
|}
*/
fn new_imperative_raj(
    polarity: &str,
    preverb: &Option<Preverb>,
    stem: &VerbStem,
    person: &Person,
    number: &Number,
) -> VecDeque<Morpheme> {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
    // Add stem
    morphemes.push_back(Morpheme {
        kind: MorphemeKind::Stem(stem.clone(), root.clone()),
        // base: root.clone(),
    });

    // Add negative prefix
    if polarity == "мы" {
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

    // Add
    if !(preverb.is_some() && Person::Third == *person) {
        let marker = PersonMarker::new(
            *person,
            // If there is a preverb present, the third person marker is not present
            if (person, number) == (&Person::Third, &Number::Plural) {
                Number::Singular
            } else {
                *number
            },
            Case::Ergative,
        );
        let m = Morpheme::new_person_marker(&marker);
        morphemes.push_front(m);
    }
    morphemes
}
fn table_imperative_raj(desc: &template::TemplateDesc) -> String {
    let mut table = Wikitable::new();
    table.add("Ре-кӀэ унафэ наклоненэ".to_owned());
    let pronouns = match &desc.transitivity {
        Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
        Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
    };
    for pronoun in pronouns.iter() {
        table.add(pronoun.to_string());
    }

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            for person in &[Person::First, Person::Second, Person::Third] {
                let morphemes =
                    new_imperative_raj(polarity, &desc.preverb, &desc.stem, person, number);
                let string = evaluation::evaluate_morphemes(&morphemes);
                table.add(string);
            }
        }
    }
    table.to_string()
}

fn table_indicative(desc: &template::TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();

    let mut table = Wikitable::new();
    table.add("ЗэраӀуатэ наклоненэ".to_owned());
    let subject_case = &desc.transitivity.subject_case();
    let pronouns = match &desc.transitivity {
        Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
        Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
    };
    for pronoun in pronouns.iter() {
        table.add(pronoun.to_string());
    }

    table.add_row();
    table.add("ит зэман – щыӀэныгъэ".to_owned());
    for number in &[Number::Singular, Number::Plural] {
        for person in &[Person::First, Person::Second, Person::Third] {
            let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
            morphemes.push_back(Morpheme {
                kind: MorphemeKind::Stem(desc.stem.clone(), root.clone()),
                // base: root.clone(),
            });
            morphemes.push_back(Morpheme::new_generic("р"));

            morphemes.push_front(Morpheme::new_generic("о"));

            // Add absolutive person marker
            if subject_case == &Case::Ergative {
                let marker = PersonMarker::new(*person, *number, Case::Ergative);
                let m = Morpheme::new_person_marker(&marker);
                morphemes.push_front(m);
            }
            // Add preverb
            if let Some(preverb) = desc.preverb.clone() {
                let m = Morpheme::new_preverb(&preverb);
                morphemes.push_front(m);
            }
            if subject_case == &Case::Absolutive {
                if (person) != (&Person::Third) {
                    let marker = PersonMarker::new(*person, *number, Case::Absolutive);
                    let m = Morpheme::new_person_marker(&marker);
                    morphemes.push_front(m);
                }
            }
            let string = evaluation::evaluate_morphemes(&morphemes);
            table.add(string);
        }
    }
    table.to_string()
}

fn create_template(desc: template::TemplateDesc) -> String {
    let mut result = "".to_string();
    result += &format!("<!-- Template:Wt/kbd/{} -->\n", desc.original_string);

    // Инфинитив (масдар)
    result += &table_masdar(&desc);
    result += "\n-\n";

    // Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа
    result += &table_masdar_personal(&desc);
    result += "\n-\n";
    // унафэ наклоненэ
    result += &table_imperative(&desc);
    result += "\n-\n";

    // Ре-кӀэ унафэ наклоненэ
    result += &table_imperative_raj(&desc);
    result += "\n-\n";

    result += &table_indicative(&desc);
    result += "\n-\n";

    result += "|}<noinclude>\n[[Category:Wt/kbd]]\n</noinclude>";

    result
}

/*
    Ideas about the strucutre.
    - Having several modes. I want this projec to be quite flexible. The idea is not only to support wikipsalhalhe, but also other projects in the future if need be.
        Right now wikipsalhalhe is the main focus, but it should be extensible to other projects.

    1. Template extraction:
        We give the engine a template string. It extract the necessary information from it.
    2.
*/
fn main() {
    let possible_templates = [
        "спр-лъэмыӏ-0-0д-э",
        "спр-лъэмыӏ-0-0д-ы",
        "спр-лъэмыӏ-0-0л-ы",
        "спр-лъэмыӏ-0-0т-ы",
        "спр-лъэмыӏ-0-бд-э",
        "спр-лъэмыӏ-0-бдэа-э",
        "спр-лъэмыӏ-0-бт-ы",
        "спр-лъэмыӏ-0-бй-ы",
        "спр-лъэӏ-0-дблэа-ы",
        "спр-лъэӏ-0-дбд-ы",
        "спр-лъэӏ-0-жь0й-ы",
        "спр-лъэӏ-0-д0д-э",
        "спр-лъэӏ-0-убт-ы",
        "спр-лъэӏ-0-д0д-ы",
        "спр-лъэӏ-0-д0л-ы",
        // "спр-лъэмыӏ-е-бд-ы",
    ];

    // those are only test roots so that one can visually test the tables better.
    // In many cases the resulting table won't correspond to real words.
    let mut test_roots: std::collections::HashMap<&str, &str>;
    test_roots = std::collections::HashMap::new();
    test_roots.insert("0д", "в");
    test_roots.insert("0л", "гъу");
    test_roots.insert("0т", "гъ");

    test_roots.insert("бд", "гупсыс");
    test_roots.insert("бдэа", "лэжь");
    test_roots.insert("бт", "дыхьэшх");
    test_roots.insert("бй", "же");

    test_roots.insert("дблэа", "лъагъу");
    test_roots.insert("дбд", "тхьэщI");
    test_roots.insert("жь0й", "и");
    test_roots.insert("д0д", "щI");
    test_roots.insert("убт", "ух");
    test_roots.insert("д0д", "хь");
    test_roots.insert("д0л", "ху");

    // спр-лъэӏ-зэхэ-д0д-ы
    let template = "спр-лъэмыӏ-0-0д-ы"; // tr. base. vl. e.g. хьын
                                        // let template = "спр-лъэмыӏ-0-0д-ы"; // intr. base. vl. e.g. плъэн
    let template_desc = template::create_template_from_string(template.to_owned()).unwrap();
    let template_str = create_template(template_desc);

    if let Some(root) = test_roots.get(template::get_root_str(template)) {
        let result = template_str.replace("{{{псалъэпкъ}}}", root);
        println!("{}", result);
    } else {
        println!("{}", template_str);
    }
}
