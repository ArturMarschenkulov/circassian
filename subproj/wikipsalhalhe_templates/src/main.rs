// спр-лъэмыӏ-0-0д-э

enum PreverbForm {
    Full,
    Reduced,
    BeforeVowel,
}

struct Preverb {
    form: PreverbForm,
    base: String,
}
#[derive(PartialEq)]
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
#[derive(PartialEq)]
enum SoundForm {
    Base,
    BeforeUnvoiced,
    BeforeVoiced,
    BeforeVowel,
    /// Transitive verbs which have the negative prefix мы- and are base (without preverb)
    /// take the absolutive markers (at least they look like that), except the third person.
    /// NOTE: This is only the case if there is no prefix before that.
    /// пхьы vs умыхь
    NegativePrefixBase,
}

struct PersonMarker {
    person: Person,
    number: Number,
    case: PersonMarkerCase,
    form: SoundForm,
}
impl PersonMarker {
    /// Returns the "base" form of the person markers
    fn get_base(&self) -> String {
        Self::get_base_from(&self.person, &self.number, &self.case).to_string()
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
    fn to_string(&self) -> String {
        use Number::*;
        use Person::*;
        use PersonMarkerCase::*;
        let mut base_consonant = Self::get_base_from(&self.person, &self.number, &self.case);

        if &self.form == &SoundForm::NegativePrefixBase
            && self.case == Ergative
            && self.person != Third
        {
            base_consonant = Self::get_base_from(&self.person, &self.number, &Absolutive);
        }
        return base_consonant.to_owned();
    }
}

struct Wikitable {
    header: String,
    cells: Vec<Vec<String>>,
}
impl Wikitable {
    fn new() -> Self {
        Wikitable {
            header: "".to_owned(),
            cells: vec![vec![]],
        }
    }
    fn add_row(&mut self) -> () {
        self.cells.push(vec![]);
    }
    fn add(&mut self, s: String) -> () {
        let last_row = self.cells.last_mut().unwrap();
        last_row.push(s);
    }
    fn to_string(&self) -> String {
        let mut result = String::new();
        result += &"{| class=\"wikitable\"\n";
        result += &"|-\n";
        result += &format!("! {} ", self.cells[0][0]);
        for i in 1..self.cells[0].len() {
            result += &format!("!! {} ", self.cells[0][i]);
        }
        result += &"\n";
        result += &"|-\n";

        for i in 1..self.cells.len() {
            result += &format!("| {} ", self.cells[i][0]);
            for j in 1..self.cells[i].len() {
                result += &format!("|| {} ", self.cells[i][j]);
            }
            result += &"\n";
            result += &"|-\n";
        }
        result += &"|}";
        return result.to_owned();
    }
}

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
enum VowelKind {
    With,
    Without,
    Alternating,
}

/// Here is the information stored about the verb stem.
/// It is extracted from the template string.
/// In the Kabardian language itself, all stems are mostly treated the same, however because of the orthographical system
/// there are some difference how those stems are treated.
#[derive(Debug)]
struct VerbStem {
    first_consonant: ConsonantKind,
    vowel: VowelKind,
    last_consonant: ConsonantKind,
    thematic_vowel: ThematicVowel,
    string: String,
}
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug)]
struct TemplateDesc {
    transitivity: Transitivity,
    preverb: String,
    stem: VerbStem,
    original_string: String,
}

fn treat_thematic_vowel(tv: &ThematicVowel, vs: &VerbStem) -> String {
    if tv == &ThematicVowel::Y && vs.last_consonant == ConsonantKind::Labialized {
        format!("")
    } else if tv == &ThematicVowel::Y {
        format!("ы")
    } else {
        format!("э")
    }
}

fn get_masdar(desc: &TemplateDesc) -> String {
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
        table.add(format!("{}{}{}", polarity, root, infinitve_ending));
    }

    table.to_string()
}
fn get_masdar_personal(desc: &TemplateDesc) -> String {
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

    let root = "{{{псалъэпкъ}}}".to_owned();
    let thematic_vowel = treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem);
    let infinitve_ending = format!("{}н", thematic_vowel);

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
                let is_third_pl = number == Number::Plural && person == Person::Third;
                let marker = PersonMarker {
                    person,
                    number,
                    case: desc.transitivity.get_subject_case(),
                    form: if polarity != "мы" {
                        SoundForm::Base
                    } else {
                        SoundForm::NegativePrefixBase
                    },
                };
                let pl = if is_third_pl { "(хэ)" } else { "" };
                let s = format!(
                    "{}{}{}{}",
                    marker.to_string(),
                    polarity,
                    root,
                    &(infinitve_ending.clone() + &pl)
                );
                table.add(s);
            }
        }
    }
    table.to_string()
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
            let marker = PersonMarker {
                person: Person::Second,
                number,
                case: desc.transitivity.get_subject_case(),
                form: if polarity != "мы" {
                    SoundForm::Base
                } else {
                    SoundForm::NegativePrefixBase
                },
            };
            let mut s = marker.to_string();
            if &marker.person == &Person::Second && &marker.number == &Number::Singular {
                s = "".to_owned();
            }

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

                        // If the verb is base (no preverbs/prefixes) and has a consonant root, then it is treated in a special way.
                        let s = if no_vowel_in_stem && is_positive && is_singular && is_transitive {
                            treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem)
                        } else if no_vowel_in_stem && is_positive {
                            treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem)
                        } else {
                            "".to_string()
                        };
                        s

                        // if is_transitive {
                        //     if no_vowel_in_stem && is_positive {
                        //         treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem)
                        //     } else {
                        //         "".to_string()
                        //     }
                        // } else {
                        //     if no_vowel_in_stem && is_positive && is_singular {
                        //         treat_thematic_vowel(&desc.stem.thematic_vowel, &desc.stem)
                        //     } else {
                        //         "".to_string()
                        //     }
                        // }
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
                let marker = PersonMarker {
                    person,
                    number,
                    case: PersonMarkerCase::Ergative,
                    form: SoundForm::Base,
                };
                let thematic_vowel = {
                    if &desc.stem.thematic_vowel == &ThematicVowel::Y {
                        "".to_string()
                    } else {
                        format!("{}", &desc.stem.thematic_vowel)
                    }
                };
                let s = format!(
                    "{}ре{}{}{}",
                    marker.to_string(),
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
    // let root = "{{{псалъэпкъ}}}".to_owned();
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
    let ss = s.split("-").collect::<Vec<&str>>();

    // Every string must start with "спр". If this is not the case, the string is false.
    if ss[0] != "спр" {
        println!("The string does not start with 'спр'");
        return None;
    }
    let transitivity = match ss[1] {
        "лъэмыӏ" => Transitivity::Intransitive,
        "лъэӏ" => Transitivity::Transitive,
        _ => {
            println!("The second string isn't either 'лъэмыӏ' or 'лъэӏ'");
            return None;
        }
    };
    let preverb = match ss[2] {
        "0" => None,
        _ => Some(ss[2]),
    };

    let ending = match ss.last() {
        Some(&"э") => ThematicVowel::A,
        Some(&"ы") => ThematicVowel::Y,
        _ => {
            println!("The last string isn't either 'э' or 'ы'");
            return None;
        }
    };

    let (fc, v, lc) = {
        //TODO: Refactor this!!! This is very messy.

        let root = ss[3].clone();
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
        preverb: preverb.unwrap_or(&"").to_owned(),
        stem: VerbStem {
            first_consonant: fc,
            vowel: v,
            last_consonant: lc,
            thematic_vowel: ending,
            string: format!("{}{}", ss[3], ss.last().unwrap()),
        },
        original_string: s.clone(),
    };

    println!("Detected those segments in template string: {:?}", ss);
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
    let template = "спр-лъэӏ-0-д0д-ы"; // tr. base. vl. e.g. хьын
                                       // let template = "спр-лъэмыӏ-0-0д-ы"; // intr. base. vl. e.g. плъэн
    let template = create_template_from_string(template.to_owned());
}
