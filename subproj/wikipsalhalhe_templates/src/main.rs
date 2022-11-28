#![allow(dead_code, unused_variables)]
#![allow(clippy::match_like_matches_macro)]

mod evaluation;
mod ortho;
mod table;

use std::collections::VecDeque;

use table::Wikitable;

#[derive(Debug, Clone)]
enum PreverbForm {
    Full, // e.g. къэ-
    // Reduced,     // e.g. къы-
    BeforeVowel, // e.g. къ-
}

#[derive(Debug, Clone)]
struct Preverb {
    form: PreverbForm,
    base: String,
}

#[derive(Debug, Clone)]
enum MorphemeKind {
    Preverb(Preverb),
    PersonMarker(PersonMarker),
    Stem(VerbStem),
    NegationPrefix,
    RajImperative,
    Generic,
}
#[derive(Debug, Clone)]
pub struct Morpheme {
    kind: MorphemeKind,
    base: String,
}
impl Morpheme {
    fn make_generic(base: &str) -> Self {
        Morpheme {
            kind: MorphemeKind::Generic,
            base: base.to_owned(),
        }
    }
    fn make_preverb(preverb: &Preverb) -> Self {
        Morpheme {
            kind: MorphemeKind::Preverb(preverb.clone()),
            base: preverb.base.clone(),
        }
    }
    fn make_person_marker(marker: &PersonMarker) -> Self {
        Morpheme {
            kind: MorphemeKind::PersonMarker(*marker),
            base: marker.get_string(),
        }
    }
}
impl Preverb {
    fn get_string(&self, form: PreverbForm) -> String {
        let sss = match &self.base {
            // This handles the preverbs which end on 'э'
            base if base.ends_with('э') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();

                match form {
                    PreverbForm::Full => base.to_owned(),
                    // PreverbForm::Reduced => reduced + "ы",
                    PreverbForm::BeforeVowel => reduced,
                }
            }
            _ => unreachable!(),
        };
        sss
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Case {
    /// (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    /// (-м) subject of transitive verb
    Ergative,
    // /// (-м) indirect object of intransitive and transitive verbs.
    // Oblique,
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
enum SoundForm {
    Base,
    BeforeUnvoiced,
    BeforeVoiced,
    // BeforeVowel,
    /// This is only for 'я' which becomes after consonants 'а'
    AfterConsonant,
    /// Transitive verbs which have the negative prefix мы- and are base (without preverb)
    /// take the absolutive markers (at least they look like that), except the third person.
    /// NOTE: This is only the case if there is no prefix before that.
    /// пхьы vs умыхь
    NegativePrefixBase,
    // NOTE: Probably add `BeforeSonorant` as there is sometimes different behavior before <м> /m/ and <р> /r/.
}

#[derive(Debug, Clone, Copy)]
struct PersonMarker {
    person: Person,
    number: Number,
    case: Case,
    form: SoundForm,
}

impl PersonMarker {
    fn new(person: Person, number: Number, case: Case) -> Self {
        PersonMarker {
            person,
            number,
            case,
            form: SoundForm::Base,
        }
    }
}
impl PersonMarker {
    /// Returns the "base" form of the person markers
    fn get_base(&self) -> String {
        use Case::*;
        use Person::*;

        // sanity checks
        // assert_ne!(Ergative == self.case, SoundForm::AfterConsonant == self.form);
        // assert_ne!(Ergative == self.case, SoundForm::BeforeUnvoiced == self.form);
        // assert_eq!(Ergative == self.case, SoundForm::BeforeVoiced == self.form);

        let mut result = Self::get_base_from(&self.person, &self.number, &self.case);
        match (&self.case, &self.form) {
            (Ergative, SoundForm::BeforeUnvoiced) => {
                result = match result.as_str() {
                    "б" => "п",
                    "д" => "т",
                    _ => result.as_str(),
                }
                .to_owned();
            }
            (Ergative, SoundForm::BeforeVoiced) => {
                result = match result.as_str() {
                    "с" => "з",
                    "ф" => "в",
                    _ => result.as_str(),
                }
                .to_owned();
            }
            (Ergative, SoundForm::AfterConsonant) => {
                result = match result.as_str() {
                    "я" => "а",
                    _ => result.as_str(),
                }
                .to_owned();
            }
            _ => (),
        }

        if self.form == SoundForm::NegativePrefixBase
            && self.case == Ergative
            && self.person != Third
        {
            result = Self::get_base_from(&self.person, &self.number, &Absolutive);
        }

        result
    }
    fn get_base_from(person: &Person, number: &Number, case: &Case) -> String {
        use Case::*;
        use Number::*;
        use Person::*;
        let result = match (&person, &number, &case) {
            (First, Singular, Absolutive) => "сы",
            (First, Singular, Ergative) => "с",
            // (First, Singular, Oblique) => "сэ",
            (Second, Singular, Absolutive) => "у",
            (Second, Singular, Ergative) => "б",
            // (Second, Singular, Oblique) => "уэ",
            (Third, Singular, Absolutive) => "",
            (Third, Singular, Ergative) => "и",
            // (Third, Singular, Oblique) => "е",
            (First, Plural, Absolutive) => "ды",
            (First, Plural, Ergative) => "д",
            // (First, Plural, Oblique) => "дэ",
            (Second, Plural, Absolutive) => "фы",
            (Second, Plural, Ergative) => "ф",
            // (Second, Plural, Oblique) => "фэ",
            (Third, Plural, Absolutive) => "",
            (Third, Plural, Ergative) => "я",
            // (Third, Plural, Oblique) => "е",
        };

        result.to_string()
    }
    fn get_string(&self) -> String {
        self.get_base()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ConsonantKind {
    /// Refers to unvoiced consonants.
    /// Can only be in the beginning of transitive verbs.
    /// It makes the preceding consonant unvoiced.
    /// Comes from 'д'.
    Unvoiced,
    /// Refers to voiced consonants.
    /// Can only be in the beginning of transitive verbs.
    /// It makes the preceding consonant voiced.
    /// Comes from 'жъ'.
    Voiced,
    /// Refers to velar consonants.
    /// Can only be in the end.
    /// Forces an insertion of ы before a у, to differentiate between labialized consonants къу vs къыу
    /// Comes from 'т'.
    Velar,
    /// Refers to labialized consonants.
    /// Can only be in the end.
    /// There can't be an ы behind it, as it's already implicit. гуыр -> гъур
    /// Comes from 'л'.
    Labialized,
    /// Refers to consonants that are neither voiced nor unvoiced.
    /// Can only be in the end. Intransitive verbs can also have it at the beginning, because there voiceness doesn't matter.
    ///
    /// Comes from 'д'.
    Ordinary,
}
#[derive(Debug, Clone, PartialEq)]
enum VowelKind {
    With,
    Without,
    Alternating,
}

/// Here is the information stored about the verb stem.
/// It is extracted from the template string.
/// In the Kabardian language itself, all stems are mostly treated the same, however because of the orthographical system
/// there are some difference how those stems are treated.
#[derive(Debug, Clone)]
struct VerbStem {
    first_consonant: ConsonantKind,
    vowel: VowelKind,
    last_consonant: ConsonantKind,
    thematic_vowel: ThematicVowel,
    string: String,
}
#[derive(Debug, Clone, PartialEq)]
enum Transitivity {
    Transitive,
    Intransitive,
}

impl Transitivity {
    fn get_subject_case(&self) -> Case {
        match self {
            Transitivity::Transitive => Case::Ergative,
            Transitivity::Intransitive => Case::Absolutive,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
enum ThematicVowel {
    A,
    Y,
}
impl std::fmt::Display for ThematicVowel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ThematicVowel::A => write!(f, "э"),
            ThematicVowel::Y => write!(f, "ы"),
        }
    }
}
#[derive(Debug, Clone)]
struct TemplateDesc {
    transitivity: Transitivity,
    preverb: Option<Preverb>,
    stem: VerbStem,
    original_string: String,
}

/// It's basically there to treat stems ending on у which labializes the consonants before it.
/// This also indicates an implicit ы.
fn treat_thematic_vowel(tv: &ThematicVowel, vs: &VerbStem) -> String {
    match (&tv, &vs.last_consonant) {
        (ThematicVowel::Y, ConsonantKind::Labialized) => "",
        (ThematicVowel::Y, _) => "ы",
        _ => "э",
    }
    .to_owned()
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
fn get_masdar(desc: &TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let thematic_vowel = treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem);
    let infinitve_ending = format!("{}н", thematic_vowel);
    let table_name = "Инфинитив (масдар)".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    table.add("".to_owned());

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ:", polarity));

        let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
        morphemes.push_back(Morpheme {
            kind: MorphemeKind::Stem(desc.stem.clone()),
            base: root.clone(),
        });
        morphemes.push_back(Morpheme::make_generic(&infinitve_ending));

        if polarity == "мы" {
            morphemes.push_front(Morpheme {
                kind: MorphemeKind::NegationPrefix,
                base: "мы".to_owned(),
            });
        }

        if let Some(preverb) = desc.preverb.clone() {
            morphemes.push_front(Morpheme::make_preverb(&preverb));
        }
        table.add(evaluation::evaluate_morphemes(&morphemes));
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
fn get_masdar_personal(desc: &TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let thematic_vowel = treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem);
    let infinitve_ending = format!("{}н", thematic_vowel);

    let table_name = "Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);

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
                let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
                morphemes.push_back(Morpheme {
                    kind: MorphemeKind::Stem(desc.stem.clone()),
                    base: root.clone(),
                });
                morphemes.push_back(Morpheme::make_generic(&infinitve_ending));

                if polarity == "мы" {
                    morphemes.push_front(Morpheme {
                        kind: MorphemeKind::NegationPrefix,
                        base: "мы".to_owned(),
                    });
                }

                let case = &desc.transitivity.get_subject_case();

                if case == &Case::Ergative {
                    let erg_marker = PersonMarker::new(*person, *number, *case);
                    morphemes.push_front(Morpheme::make_person_marker(&erg_marker));
                };

                if let Some(preverb) = desc.preverb.clone() {
                    morphemes.push_front(Morpheme::make_preverb(&preverb));
                }

                let (_person, _number) = if desc.transitivity == Transitivity::Transitive {
                    (Person::Third, Number::Singular)
                } else {
                    (*person, *number)
                };
                let abs_marker = PersonMarker::new(_person, _number, *case);

                morphemes.push_front(Morpheme::make_person_marker(&abs_marker));

                let s = evaluation::evaluate_morphemes(&morphemes);
                table.add(s);
            }
        }
    }

    // let s = morphemes
    //     .iter()
    //     .map(|m| m.base.clone())
    //     .collect::<Vec<String>>()
    //     .join("-");
    // println!("{}", s);
    // evaluate_morphemes(&morphemes);
    table.to_string()
}

// enum Phone {
//     Consonant(Consonant),
//     Vowel(Vowel),
// }

fn get_imperative(desc: &TemplateDesc) -> String {
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
    let root = "{{{псалъэпкъ}}}".to_owned();
    let table_name = "унафэ наклоненэ".to_owned();
    // let thematic_vowel = treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem);

    let mut table = Wikitable::new();
    table.add(table_name);
    for pronoun in ["уэ", "фэ"].iter() {
        table.add(pronoun.to_string());
    }

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
            morphemes.push_back(Morpheme {
                kind: MorphemeKind::Stem(desc.stem.clone()),
                base: root.clone(),
            });

            if polarity == "мы" {
                morphemes.push_front(Morpheme {
                    kind: MorphemeKind::NegationPrefix,
                    base: "мы".to_owned(),
                });
            }
            let case = &desc.transitivity.get_subject_case();
            if case == &Case::Ergative && *number == Number::Plural {
                let erg_marker = PersonMarker::new(Person::Second, *number, *case);
                morphemes.push_front(Morpheme::make_person_marker(&erg_marker));
            };

            if let Some(preverb) = desc.preverb.clone() {
                morphemes.push_front(Morpheme::make_preverb(&preverb));
            }
            let (abs_person, abs_number) = if desc.transitivity == Transitivity::Transitive {
                (Person::Third, Number::Singular)
            } else {
                (Person::Second, *number)
            };
            if (abs_person, abs_number) == (Person::Second, Number::Plural) {
                let abs_marker = PersonMarker::new(abs_person, abs_number, Case::Absolutive);
                morphemes.push_front(Morpheme::make_person_marker(&abs_marker));
            }
            let s = evaluation::evaluate_morphemes(&morphemes);
            table.add(s);
        }
    }
    table.to_string()
}

fn get_imperative_raj(desc: &TemplateDesc) -> String {
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
    let root = "{{{псалъэпкъ}}}".to_owned();

    let mut table = Wikitable::new();
    table.add("Ре-кӀэ унафэ наклоненэ".to_owned());
    for pronoun in ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"].iter() {
        table.add(pronoun.to_string());
    }

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            for person in &[Person::First, Person::Second, Person::Third] {
                let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

                morphemes.push_back(Morpheme {
                    kind: MorphemeKind::Stem(desc.stem.clone()),
                    base: root.clone(),
                });

                if polarity == "мы" {
                    morphemes.push_front(Morpheme {
                        kind: MorphemeKind::NegationPrefix,
                        base: "мы".to_owned(),
                    });
                }
                morphemes.push_front(Morpheme {
                    kind: MorphemeKind::RajImperative,
                    base: "ре".to_owned(),
                });
                if let Some(preverb) = desc.preverb.clone() {
                    morphemes.push_front(Morpheme::make_preverb(&preverb));
                }

                let marker = PersonMarker::new(
                    *person,
                    if (person, number) == (&Person::Third, &Number::Plural) {
                        Number::Singular
                    } else {
                        *number
                    },
                    Case::Ergative,
                );

                // If there is a preverb present, the third person marker is not present
                if !(desc.preverb.is_some() && Person::Third == *person) {
                    morphemes.push_front(Morpheme::make_person_marker(&marker));
                }

                let s = evaluation::evaluate_morphemes(&morphemes);
                table.add(s);
            }
        }
    }
    table.to_string()
}
fn create_template(desc: TemplateDesc) -> String {
    let mut result = "".to_string();
    result += &format!("<!-- Template:Wt/kbd/{} -->\n", desc.original_string);

    // Инфинитив (масдар)
    result += &get_masdar(&desc);
    result += "\n-\n";

    // Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа
    result += &get_masdar_personal(&desc);
    result += "\n-\n";
    // унафэ наклоненэ
    result += &get_imperative(&desc);
    result += "\n-\n";

    // Ре-кӀэ унафэ наклоненэ
    result += &get_imperative_raj(&desc);
    result += "\n-\n";

    result += "|}<noinclude>\n[[Category:Wt/kbd]]\n</noinclude>";
    println!("{}", result);

    result
}

fn create_template_from_string(s: String) -> Option<TemplateDesc> {
    let segments = s.split('-').collect::<Vec<&str>>();

    // Every string must start with "спр". If this is not the case, the string is false.
    if segments[0] != "спр" {
        println!("The string does not start with 'спр'");
        return None;
    }
    let transitivity = match segments[1] {
        "лъэмыӏ" => Transitivity::Intransitive,
        "лъэӏ" => Transitivity::Transitive,
        _ => {
            println!("The second string isn't either 'лъэмыӏ' or 'лъэӏ'");
            return None;
        }
    };
    let preverb = match segments[2] {
        "0" => None,
        _ => Some(Preverb {
            form: PreverbForm::Full,
            base: segments[2].to_owned(),
        }),
    };

    let ending = match segments.last() {
        Some(&"э") => ThematicVowel::A,
        Some(&"ы") => ThematicVowel::Y,
        _ => {
            println!("The last string isn't either 'э' or 'ы'");
            return None;
        }
    };

    let (fc, v, lc) = {
        //TODO: Refactor this!!! This is very messy.

        let root = segments[3].clone();
        let mut fc = ConsonantKind::Ordinary;
        let mut v = VowelKind::Without;
        let lc;

        if root.starts_with('0') {
            assert_eq!(transitivity, Transitivity::Intransitive);
            fc = ConsonantKind::Ordinary;
            v = VowelKind::Without;
        } else if root.starts_with('б') {
            assert_eq!(transitivity, Transitivity::Intransitive);
            fc = ConsonantKind::Ordinary;
            v = VowelKind::With;
        } else if root.starts_with("жь") {
            assert_eq!(transitivity, Transitivity::Transitive);
            fc = ConsonantKind::Voiced;
            if root.find('0').is_some() {
                v = VowelKind::Without;
            } else if root.find('б').is_some() {
                v = VowelKind::With;
            } else {
                panic!();
            }
        } else if root.starts_with('д') {
            assert_eq!(transitivity, Transitivity::Transitive);
            fc = ConsonantKind::Unvoiced;
            if root.find('0').is_some() {
                v = VowelKind::Without;
            } else if root.find('б').is_some() {
                v = VowelKind::With;
            } else {
                panic!();
            }
        }

        match root {
            _ if root.ends_with('д') => {
                lc = ConsonantKind::Ordinary;
            }
            _ if root.ends_with('т') => {
                lc = ConsonantKind::Velar;
            }
            _ if root.ends_with('л') => {
                lc = ConsonantKind::Labialized;
            }
            _ if root.ends_with("дэа") => {
                lc = ConsonantKind::Ordinary;
                v = VowelKind::Alternating;
            }
            _ => {
                panic!();
            }
        }
        (fc, v, lc)
    };
    let template_desc = TemplateDesc {
        transitivity,
        preverb,
        stem: VerbStem {
            first_consonant: fc,
            vowel: v,
            last_consonant: lc,
            thematic_vowel: ending,
            string: format!("{}{}", segments[3], segments.last().unwrap()),
        },
        original_string: s.clone(),
    };
    Some(template_desc)
}

/*
    From the Email of Robert Dunwell (Rhdkabardian), the following encoding is used.
    лъэмыӏ- = intransitive (this is also used for passive verbs - this probably be separate)
    лъэӏ- = transitive
    стат- = stative
    NOTE: In the future a static transitive category will be added.
    NOTE: Passive verbs use лъэмыӏ, in the future they will get their own category.

    Transitive:

    CCC
    д = unvoiced first consonant
    жь = voiced first consonant
    The second and third consonants are as in intransitive verbs.

    Intransitive, stative, causative

    CC
    б = not used - vowel in root
    д = ordinary consinant
    дэа = root with alternating a/э (verbs in -э only)
    т = root in a velar
    л = root in a labial
    й = root in jot
    у = root in wy
    йэа = root with alternating a/э (verbs in -э only)

    Causative
    бд = causative prefix гъэ
    0д = causative prefix гъэ/гъа

*/
/*

    спр-лъэмыӏ-0-0д-э: плъэн
    спр-лъэмыӏ-0-0д-ы:
    спр-лъэмыӏ-0-0л-ы: гъун
    спр-лъэмыӏ-0-0т-ы: гъын

    спр-лъэмыӏ-0-бд-э: гупсысэн гуфIэн
    спр-лъэмыӏ-0-бдэа-э: лэжьэн
    спр-лъэмыӏ-0-бт-ы: дыхьэшхын
    спр-лъэмыӏ-0-бй-ы: жеин

    спр-лъэӏ-0-дблэа-ы: лъагъун
    спр-лъэӏ-0-дбд-ы: тхьэщIын
    спр-лъэӏ-0-жь0й-ы: ин
    спр-лъэӏ-0-д0д-э: щIэн
    спр-лъэӏ-0-убт-ы: ухын
    спр-лъэӏ-0-д0д-ы: хьын тын
    спр-лъэӏ-0-д0л-ы: хун

    спр-лъэмыӏ-е-бд-ы: еплъын

*/

/*
    Ideas about the strucutre.
    - Having several modes. I want this projec to be quite flexible. The idea is not only to support wikipsalhalhe, but also other projects in the future if need be.
        Right now wikipsalhalhe is the main focus, but it should be extensible to other projects.

    1. Template extraction:
        We give the engine a template string. It extract the necessary information from it.
    2. 
*/
fn main() {
    let template = "спр-лъэӏ-дэ-д0д-ы"; // tr. base. vl. e.g. хьын
                                        // let template = "спр-лъэмыӏ-0-0д-ы"; // intr. base. vl. e.g. плъэн
    let template = create_template_from_string(template.to_owned()).unwrap();
    create_template(template);
}
