mod table;

use std::collections::VecDeque;

use table::Wikitable;

#[derive(Debug, Clone)]
enum PreverbForm {
    Full,        // e.g. къэ-
    Reduced,     // e.g. къы-
    BeforeVowel, // e.g. къ-
}

#[derive(Debug, Clone)]
struct Preverb {
    form: PreverbForm,
    base: String,
}

enum MorphemeKind {
    Preverb(Preverb),
    PersonMarker(PersonMarker),
    Stem(VerbStem),
    NegationPrefix,
    Generic,
    Backpart,
}
struct Morpheme {
    kind: MorphemeKind,
    base: String,
}
impl Preverb {
    fn get_string(&self, form: PreverbForm) -> String {
        let sss = match &self.base {
            // This handles the preverbs which end on 'э'
            base if base.ends_with("э") => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();

                match form {
                    PreverbForm::Full => base.to_owned(),
                    PreverbForm::Reduced => reduced + "ы",
                    PreverbForm::BeforeVowel => reduced,
                }
            }
            _ => unreachable!(),
        };
        sss
    }
}
#[derive(Clone, Copy, PartialEq)]
enum PersonMarkerCase {
    /// (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    /// (-м) subject of transitive verb
    Ergative,
    /// (-м) indirect object of intransitive and transitive verbs.
    Oblique,
}

/// A struct that indicates the number of a noun or verb.
#[derive(Clone, Copy, PartialEq)]
enum Number {
    Singular,
    Plural,
}
/// A struct that indicates the person of a verb.
#[derive(Clone, Copy, PartialEq)]
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
    BeforeVowel,
    /// This is only for 'я' which becomes after consonants 'а'
    AfterConsonant,
    /// Transitive verbs which have the negative prefix мы- and are base (without preverb)
    /// take the absolutive markers (at least they look like that), except the third person.
    /// NOTE: This is only the case if there is no prefix before that.
    /// пхьы vs умыхь
    NegativePrefixBase,
    // NOTE: Probably add `BeforeSonorant` as there is sometimes different behavior before <м> /m/ and <р> /r/.
}

#[derive(Clone, Copy)]
struct PersonMarker {
    person: Person,
    number: Number,
    case: PersonMarkerCase,
    form: SoundForm,
}
impl PersonMarker {
    /// Returns the "base" form of the person markers
    fn get_base(&self) -> String {
        use Number::*;
        use Person::*;
        use PersonMarkerCase::*;

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

        if &self.form == &SoundForm::NegativePrefixBase
            && self.case == Ergative
            && self.person != Third
        {
            result = Self::get_base_from(&self.person, &self.number, &Absolutive);
        }

        result.to_string()
    }
    fn get_base_from(person: &Person, number: &Number, case: &PersonMarkerCase) -> String {
        use Number::*;
        use Person::*;
        use PersonMarkerCase::*;
        let result = match (&person, &number, &case) {
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
        let base_consonant = self.get_base();
        return base_consonant.to_owned();
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
    fn get_subject_case(&self) -> PersonMarkerCase {
        match self {
            Transitivity::Transitive => PersonMarkerCase::Ergative,
            Transitivity::Intransitive => PersonMarkerCase::Absolutive,
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
    table.add(table_name.clone());
    table.add("".to_owned());

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ:", polarity));

        let preverb = desc
            .preverb
            .as_ref()
            .map(|p| p.get_string(PreverbForm::Full))
            .unwrap_or("".to_owned());
        table.add(format!(
            "{}{}{}{}",
            preverb, polarity, root, infinitve_ending
        ));
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
fn get_masdar_personal_(desc: &TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();

    let table_name = "Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name.clone());
    for pronoun in ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"].iter() {
        table.add(pronoun.to_string());
    }

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in vec![Number::Singular, Number::Plural] {
            for person in vec![Person::First, Person::Second, Person::Third] {
                use ConsonantKind::*;
                use PersonMarkerCase::*;
                let thematic_vowel = treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem);
                let infinitve_ending = format!("{}н", thematic_vowel);

                let case = desc.transitivity.get_subject_case();
                let is_second_singular = (person, number) == (Person::Second, Number::Singular);

                let form = match (
                    &polarity,
                    &case,
                    &desc.stem.first_consonant,
                    &desc.preverb.is_some(),
                ) {
                    (&"мы", Ergative, _, false) => SoundForm::NegativePrefixBase,
                    (&"мы", Ergative, _, true) => {
                        if is_second_singular {
                            SoundForm::NegativePrefixBase
                        } else {
                            SoundForm::BeforeVoiced
                        }
                    }
                    (&"", Ergative, Unvoiced, _) => SoundForm::BeforeUnvoiced,
                    (&"", Ergative, Voiced, _) => SoundForm::BeforeVoiced,
                    (&"", _, _, _) => SoundForm::Base,
                    _ => panic!("Unknown case"),
                };
                let marker = PersonMarker {
                    person,
                    number,
                    case,
                    form,
                };

                let number_suffix = match (&number, &person) {
                    (Number::Plural, Person::Third) => Some("(хэ)"),
                    _ => None,
                }
                .to_owned();

                let preverb =
                    desc.preverb
                        .as_ref()
                        .map(|p| {
                            let pf = match (person, number, polarity) {
                                (Person::Second, Number::Singular, "мы")
                                | (Person::Third, _, _) => PreverbForm::BeforeVowel,
                                _ => PreverbForm::Full,
                            };
                            p.get_string(pf)
                        })
                        .unwrap_or("".to_owned());

                let marker_string = if desc.preverb.is_some() && marker.get_string() == "я" {
                    "а".to_owned()
                } else {
                    marker.get_string()
                };

                let part_front = format!("{}{}{}", preverb, marker_string, polarity);
                let part_back = format!(
                    "{}",
                    &(infinitve_ending.clone() + &number_suffix.unwrap_or(""))
                );

                let s = format!("{}{}{}", part_front, root, part_back);
                table.add(s);
            }
        }
    }
    table.to_string()
}
fn get_masdar_personal(desc: &TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let thematic_vowel = treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem);
    let infinitve_ending = format!("{}н", thematic_vowel);

    let table_name = "Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name.clone());

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

        for number in vec![Number::Singular, Number::Plural] {
            for person in vec![Person::First, Person::Second, Person::Third] {
                use ConsonantKind::*;
                use PersonMarkerCase::*;

                let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
                morphemes.push_back(Morpheme {
                    kind: MorphemeKind::Stem(desc.stem.clone()),
                    base: root.clone(),
                });
                morphemes.push_back(Morpheme {
                    kind: MorphemeKind::Generic,
                    base: infinitve_ending.clone(),
                });

                if polarity == "мы" {
                    morphemes.push_front(Morpheme {
                        kind: MorphemeKind::NegationPrefix,
                        base: "мы".to_owned(),
                    });
                }

                let case = &desc.transitivity.get_subject_case();

                let erg_marker = if case == &Ergative {
                    Some(PersonMarker {
                        person,
                        number,
                        case: case.clone(),
                        form: SoundForm::Base,
                    })
                } else {
                    None
                };

                if let Some(erg_marker) = erg_marker {
                    morphemes.push_front(Morpheme {
                        kind: MorphemeKind::PersonMarker(erg_marker),
                        base: erg_marker.get_base(),
                    });
                }

                if let Some(preverb) = desc.preverb.clone() {
                    morphemes.push_front(Morpheme {
                        kind: MorphemeKind::Preverb(preverb.clone()),
                        base: preverb.base.clone(),
                    });
                }

                let (_person, _number) = if desc.transitivity == Transitivity::Transitive {
                    (Person::Third, Number::Singular)
                } else {
                    (person, number)
                };
                let abs_marker = PersonMarker {
                    person: _person,
                    number: _number,
                    case: PersonMarkerCase::Absolutive,
                    form: SoundForm::Base,
                };

                morphemes.push_front(Morpheme {
                    kind: MorphemeKind::PersonMarker(abs_marker),
                    base: abs_marker.get_base(),
                });

                let s = evaluate_morphemes(&morphemes);
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

fn evaluate_morphemes(morphemes: &VecDeque<Morpheme>) -> String {
    let mut result = String::new();
    for (i, morpheme) in morphemes.iter().enumerate() {
        let morpheme_last = morphemes.get(i.checked_sub(1).unwrap_or(0));
        let morpheme_next = morphemes.get(i + 1);
        let is_first_morpheme = morpheme_last.is_none();
        let is_last_morpheme = morpheme_next.is_none();

        match morpheme.kind {
            MorphemeKind::PersonMarker(marker) => {
                let mut m = marker.clone();

                let ss = match marker.case {
                    PersonMarkerCase::Ergative => {
                        let morpheme_last_kind = morpheme_last.map(|x| &x.kind);
                        let morpheme_next_kind = morpheme_next.map(|x| &x.kind);
                        let is_third_plural =
                            (marker.person, marker.number) == (Person::Third, Number::Plural);
                        let is_second_singular =
                            (marker.person, marker.number) == (Person::Second, Number::Singular);

                        let x = if is_third_plural {
                            match morpheme_last_kind {
                                Some(..) => SoundForm::AfterConsonant,
                                None => SoundForm::Base,
                            }
                        } else {
                            match (morpheme_last_kind, morpheme_next_kind) {
                                (None, Some(MorphemeKind::NegationPrefix)) => {
                                    SoundForm::NegativePrefixBase
                                }
                                (Some(..), Some(MorphemeKind::NegationPrefix)) => {
                                    let is_second_singular = (marker.person, marker.number)
                                        == (Person::Second, Number::Singular);
                                    if is_second_singular {
                                        SoundForm::NegativePrefixBase
                                    } else {
                                        SoundForm::BeforeVoiced
                                    }
                                }
                                (_, Some(MorphemeKind::Stem(stem))) => match stem.first_consonant {
                                    ConsonantKind::Unvoiced => SoundForm::BeforeUnvoiced,
                                    ConsonantKind::Voiced => SoundForm::BeforeVoiced,
                                    _ => unreachable!(),
                                },
                                (None, Some(..)) => SoundForm::BeforeVoiced,
                                (_, None) => unreachable!(),
                                _ => unreachable!(),
                            }
                        };
                        m.form = x;
                        m.get_base()
                    }
                    PersonMarkerCase::Absolutive => {
                        m.form = SoundForm::Base;
                        m.get_base()
                    }
                    PersonMarkerCase::Oblique => unimplemented!(),
                };
                result += &ss;
            }
            MorphemeKind::Stem(ref stem) => {
                result += &morpheme.base;
            }
            MorphemeKind::Preverb(ref preverb) => {
                let mut p = preverb.clone();

                match morpheme_next.unwrap().kind {
                    MorphemeKind::PersonMarker(marker) => {
                        match (marker.person, marker.number, marker.case) {
                            (Person::Third, _, PersonMarkerCase::Ergative) => {
                                p.form = PreverbForm::BeforeVowel;
                            }
                            (Person::Second, Number::Singular, PersonMarkerCase::Ergative) => {
                                let morpheme_next_next = morphemes.get(i + 2).unwrap();
                                if let MorphemeKind::NegationPrefix = morpheme_next_next.kind {
                                    p.form = PreverbForm::BeforeVowel;
                                } else {
                                    p.form = PreverbForm::Full;
                                }
                            }
                            _ => {
                                p.form = PreverbForm::Full;
                            }
                        }
                    }
                    _ => unimplemented!(),
                }
                result += &preverb.get_string(p.form);
            }
            MorphemeKind::NegationPrefix => {
                result += &"мы".to_owned();
            }
            MorphemeKind::Generic => {
                result += &morpheme.base;
            }
            _ => unimplemented!(),
        }
    }
    format!("{}", result)
}

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

    let mut table = Wikitable::new();
    table.add(table_name);
    for pronoun in ["уэ", "фэ"].iter() {
        table.add(pronoun.to_string());
    }

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in vec![Number::Singular, Number::Plural] {
            let case = desc.transitivity.get_subject_case();
            use ConsonantKind::*;
            use PersonMarkerCase::*;
            let form = match (&polarity, &case, &desc.stem.first_consonant) {
                (&"мы", _, _) => SoundForm::NegativePrefixBase,
                (&"", Ergative, Unvoiced) => SoundForm::BeforeUnvoiced,
                (&"", Ergative, Voiced) => SoundForm::BeforeVoiced,
                (&"", _, _) => SoundForm::Base,
                _ => panic!("Unknown case"),
            };
            let marker = PersonMarker {
                person: Person::Second,
                number,
                case,
                form,
            };
            // The second person singular imperative doesn't take a person marker
            let s = match (&marker.person, &marker.number) {
                (&Person::Second, &Number::Singular) => "".to_owned(),
                _ => marker.get_string(),
            };

            // The thematic vowel 'э' is always there,
            // however 'ы' stays only there if the form is monosyllabic
            let thematic_vowel = {
                let r = match &desc.stem.thematic_vowel {
                    ThematicVowel::A => {
                        format!("{}", &desc.stem.thematic_vowel)
                    }
                    ThematicVowel::Y => {
                        let no_vowel_in_stem = &desc.stem.vowel == &VowelKind::Without;
                        let is_positive = polarity == "";
                        let is_singular = number == Number::Singular;
                        let is_transitive = &desc.transitivity == &Transitivity::Transitive;

                        // The thematic vowel 'ы' is
                        // If the verb is base (no preverbs/prefixes) and has a consonant root, then it is treated in a special way.
                        let s = if no_vowel_in_stem && is_positive && is_singular && is_transitive {
                            treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem)
                        } else if no_vowel_in_stem && is_positive {
                            treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem)
                        } else {
                            "".to_string()
                        };
                        s
                    }
                };
                r
            };

            let s = format!("{}{}{}{}", s, polarity, root, thematic_vowel);
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
        for number in vec![Number::Singular, Number::Plural] {
            for person in vec![Person::First, Person::Second, Person::Third] {
                // All person markers are in their base ergative form, however the second person singular is seamingly in their absolutive form.
                // It's 'у-' instead of the expected 'б-' form. Most likely it's because of phonological reasons, because it preceeds 'р' /r/.
                let case = match (&person, &number) {
                    (&Person::Second, &Number::Singular) => PersonMarkerCase::Absolutive,
                    _ => PersonMarkerCase::Ergative,
                };

                // The third person can't be in the plural form. The singular is used.
                let number = match &person {
                    &Person::Third => Number::Singular,
                    _ => number,
                };

                let marker = PersonMarker {
                    person,
                    number,
                    case,
                    form: SoundForm::Base,
                };

                // The thematic vowel 'ы' is never present in such a form
                let thematic_vowel = {
                    let mut thematic_vowel = "".to_owned();
                    if &desc.stem.thematic_vowel == &ThematicVowel::A {
                        thematic_vowel = desc.stem.thematic_vowel.to_string();
                    }
                    thematic_vowel
                };

                let s = format!(
                    "{}ре{}{}{}",
                    marker.get_string(),
                    polarity,
                    root,
                    thematic_vowel
                );
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

    return result;
}

fn create_template_from_string(s: String) -> Option<String> {
    let segments = s.split("-").collect::<Vec<&str>>();

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
        let mut lc = ConsonantKind::Ordinary;

        if root.starts_with("0") {
            assert_eq!(transitivity, Transitivity::Intransitive);
            fc = ConsonantKind::Ordinary;
            v = VowelKind::Without;
        } else if root.starts_with("б") {
            assert_eq!(transitivity, Transitivity::Intransitive);
            fc = ConsonantKind::Ordinary;
            v = VowelKind::With;
        } else if root.starts_with("жь") {
            assert_eq!(transitivity, Transitivity::Transitive);
            fc = ConsonantKind::Voiced;
            if root.find("0").is_some() {
                v = VowelKind::Without;
            } else if root.find("б").is_some() {
                v = VowelKind::With;
            } else {
                panic!();
            }
        } else if root.starts_with("д") {
            assert_eq!(transitivity, Transitivity::Transitive);
            fc = ConsonantKind::Unvoiced;
            if root.find("0").is_some() {
                v = VowelKind::Without;
            } else if root.find("б").is_some() {
                v = VowelKind::With;
            } else {
                panic!();
            }
        }

        match root {
            _ if root.ends_with("д") => {
                lc = ConsonantKind::Ordinary;
            }
            _ if root.ends_with("т") => {
                lc = ConsonantKind::Velar;
            }
            _ if root.ends_with("л") => {
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
        transitivity: transitivity,
        preverb: preverb,
        stem: VerbStem {
            first_consonant: fc,
            vowel: v,
            last_consonant: lc,
            thematic_vowel: ending,
            string: format!("{}{}", segments[3], segments.last().unwrap()),
        },
        original_string: s.clone(),
    };

    println!("Detected those segments in template string: {:?}", segments);
    println!("{:#?}", template_desc);
    let template = create_template(template_desc);
    return Some(template);
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
fn main() {
    let template = "спр-лъэӏ-дэ-д0л-ы"; // tr. base. vl. e.g. хьын
                                        // let template = "спр-лъэмыӏ-0-0д-э"; // intr. base. vl. e.g. плъэн
    let template = create_template_from_string(template.to_owned());
}
