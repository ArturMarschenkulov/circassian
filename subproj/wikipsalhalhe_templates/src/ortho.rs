/// This module handles the orthography of the language.
/// The goal is to convert every letter into a unit with useful meta-data.
///
/// The main struct is `Letter`, which contains the letter itself, and a `LetterKind`.

/// `Letter` is a struct which contains the letter itself, and a `LetterKind`.
#[derive(Debug, Clone, PartialEq)]
pub struct Letter {
    pub kind: LetterKind,
    base: String,
}

/// `LetterKind` is an enum which contains the kind of the letter.
/// It can be a consonant, a vowel, or a combination of consonant and vowel.
#[derive(Debug, Clone, PartialEq)]
pub enum LetterKind {
    /// Represents a consonant letter.
    Consonant(Consonant),
    /// Represents a vowel letter.
    Vowel(Vowel),
    /// Represents special letters which are a combination of consonant and vowel
    /// Those are 'я', 'е', 'и', 'у', 'о'.
    /// Tese are represented as below:
    /// 'я' = 'йа',
    /// 'е' = 'йэ',
    /// 'и' = 'йы',
    /// 'у' = 'уы',
    /// 'о' = 'уэ', (actually it's rather'эу')
    ///
    ///
    Combi(Consonant, Vowel),
}
impl Letter {
    pub fn voiceness(&self) -> Voiceness {
        match &self.kind {
            LetterKind::Consonant(c) => c.voiceness,
            LetterKind::Vowel(..) => Voiceness::Voiced,
            LetterKind::Combi(..) => Voiceness::Voiced,
        }
    }

    pub fn is_vowel(&self) -> bool {
        match &self.kind {
            LetterKind::Vowel(..) => true,
            _ => false,
        }
    }
    pub fn is_consonant(&self) -> bool {
        match &self.kind {
            LetterKind::Consonant(..) => true,
            _ => false,
        }
    }
    pub fn is_combi(&self) -> bool {
        match &self.kind {
            LetterKind::Combi(..) => true,
            _ => false,
        }
    }
    pub fn is_consonant_or_combi(&self) -> bool {
        self.is_consonant() || self.is_combi()
    }
    pub fn is_vowel_or_combi(&self) -> bool {
        self.is_vowel() || self.is_combi()
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = match &self.kind {
            LetterKind::Consonant(c) => c.to_string(),
            LetterKind::Vowel(v) => v.to_string(),
            LetterKind::Combi(c, v) => {
                let x = combine_to_combi(&c.to_string(), &v.to_string())
                    .expect("Already checked that it is a combi");
                x.to_string()
            }
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vowel {
    pub kind: VowelKind,
    base: String,
}
impl Vowel {
    fn new(kind: VowelKind) -> Self {
        let base = match kind {
            VowelKind::AA => "а",
            VowelKind::A => "э",
            VowelKind::Y => "ы",
        }
        .to_owned();
        Vowel { kind, base }
    }
}

impl TryFrom<String> for Vowel {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "а" => Ok(Self::new(VowelKind::AA)),
            "э" => Ok(Self::new(VowelKind::A)),
            "ы" => Ok(Self::new(VowelKind::Y)),
            _ => Err(format!("{} can not be transfromed into a `Vowel`", s)),
        }
    }
}
impl std::fmt::Display for Vowel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = match self.kind {
            VowelKind::AA => "а",
            VowelKind::A => "э",
            VowelKind::Y => "ы",
        };
        write!(f, "{}", x)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum VowelKind {
    /// Represents the 'a' sound
    AA,
    /// Represents the 'э' sound
    A,
    /// Represents the 'ы' sound
    Y,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Voiceness {
    Voiceless,
    Voiced,
    Ejective,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consonant {
    pub place: Place,
    pub manner: Manner,
    pub voiceness: Voiceness,
    pub is_labialized: bool,
    pub base: String,
}
impl Consonant {
    fn new(
        place: Place,
        manner: Manner,
        voiceness: Voiceness,
        is_labialized: bool,
        base: String,
    ) -> Result<Self, String> {
        Ok(Consonant {
            place,
            manner,
            voiceness,
            is_labialized,
            base,
        })
    }
    /// Returns `true` if the consonant is a labial plosive, e.g. 'п' or 'б'.
    pub fn is_labial_plosive(&self) -> bool {
        self.is_place_and_manner(Place::Labial, Manner::Plosive)
    }

    pub fn is_place_and_manner(&self, place: Place, manner: Manner) -> bool {
        self.manner == manner && self.place == place
    }
    pub fn is_manner(&self, manner: Manner) -> bool {
        self.manner == manner
    }
    pub fn is_place(&self, place: Place) -> bool {
        self.place == place
    }

    pub fn is_nasal(&self) -> bool {
        self.is_manner(Manner::Nasal)
    }
    pub fn is_trill(&self) -> bool {
        self.is_manner(Manner::Trill)
    }
    /// Returns `true` if the consonant is a velar, uvular or pharyngeal plosive, aka it needs an 'ы' before 'у'.
    pub fn needs_epenthetic_y(&self) -> bool {
        use Place::*;
        [Velar, Uvular, Glottal].contains(&self.place)
    }
}

impl TryFrom<String> for Consonant {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        use Manner::*;
        use Place::*;
        use Voiceness::*;
        match s.as_str() {
            // Nasals
            "м" => Consonant::new(Labial, Nasal, Voiced, false, s.to_owned()),
            "н" => Consonant::new(Alveolar, Nasal, Voiced, false, s.to_owned()),
            // Plosives Unvoiced
            "п" => Consonant::new(Labial, Plosive, Voiceless, false, s.to_owned()),
            "т" => Consonant::new(Alveolar, Plosive, Voiceless, false, s.to_owned()),
            "к" => Consonant::new(Velar, Plosive, Voiceless, false, s.to_owned()),
            "къ" => Consonant::new(Uvular, Plosive, Voiceless, false, s.to_owned()),
            "I" => Consonant::new(Glottal, Plosive, Voiceless, false, s.to_owned()),
            "Iу" => Consonant::new(Glottal, Plosive, Voiceless, true, s.to_owned()),
            // Plosives Unvoiced Labialized
            "ку" => Consonant::new(Velar, Plosive, Voiceless, true, s.to_owned()),
            "кьу" => Consonant::new(Uvular, Plosive, Voiceless, true, s.to_owned()),

            // Plosives Voiced
            "б" => Consonant::new(Labial, Plosive, Voiced, false, s.to_owned()),
            "д" => Consonant::new(Alveolar, Plosive, Voiced, false, s.to_owned()),

            // Plosives Voiced Labialized
            "гу" => Consonant::new(Velar, Plosive, Voiced, true, s.to_owned()),
            // Plosives Ejective
            "пI" => Consonant::new(Labial, Plosive, Ejective, false, s.to_owned()),
            "тI" => Consonant::new(Alveolar, Plosive, Ejective, false, s.to_owned()),
            // Plosives Ejective Labialized
            "кIу" => Consonant::new(Velar, Plosive, Ejective, true, s.to_owned()),
            // Affricates Unvoiced
            "ц" => Consonant::new(Alveolar, Affricative, Voiceless, false, s.to_owned()),
            "ч" => Consonant::new(PostAlveolar, Affricative, Voiceless, false, s.to_owned()),
            "кхъ" => Consonant::new(Uvular, Affricative, Voiceless, false, s.to_owned()),
            // Affricates Unvoiced Labialized
            "кхъу" => Consonant::new(Uvular, Affricative, Voiceless, true, s.to_owned()),
            // Affricates Voiced
            "дз" => Consonant::new(Alveolar, Affricative, Voiced, false, s.to_owned()),
            "дж" => Consonant::new(PostAlveolar, Affricative, Voiced, false, s.to_owned()),
            // Affricates Ejecitive
            "цI" => Consonant::new(Alveolar, Affricative, Ejective, false, s.to_owned()),
            "кI" => Consonant::new(PostAlveolar, Affricative, Ejective, false, s.to_owned()),
            // Fricatives Unvoiced
            "ф" => Consonant::new(Labial, Fricative, Voiceless, false, s.to_owned()),
            "с" => Consonant::new(Alveolar, Fricative, Voiceless, false, s.to_owned()),
            "лъ" => Consonant::new(Lateral, Fricative, Voiceless, false, s.to_owned()),
            "ш" => Consonant::new(PostAlveolar, Fricative, Voiceless, false, s.to_owned()),
            "щ" => Consonant::new(Palatal, Fricative, Voiceless, false, s.to_owned()),
            "х" => Consonant::new(Velar, Fricative, Voiceless, false, s.to_owned()),
            "хъ" => Consonant::new(Uvular, Fricative, Voiceless, false, s.to_owned()),
            "хь" => Consonant::new(Pharyngeal, Fricative, Voiceless, false, s.to_owned()),
            // Fricatives Unvoiced Labialized
            "ху" => Consonant::new(Velar, Fricative, Voiceless, true, s.to_owned()),
            "хъу" => Consonant::new(Uvular, Fricative, Voiceless, true, s.to_owned()),
            // Fricatives Voiced
            "в" => Consonant::new(Labial, Fricative, Voiced, false, s.to_owned()),
            "з" => Consonant::new(Alveolar, Fricative, Voiced, false, s.to_owned()),
            "л" => Consonant::new(Lateral, Fricative, Voiced, false, s.to_owned()),
            "ж" => Consonant::new(PostAlveolar, Fricative, Voiced, false, s.to_owned()),
            "жь" => Consonant::new(Palatal, Fricative, Voiced, false, s.to_owned()),
            "г" => Consonant::new(Velar, Fricative, Voiced, false, s.to_owned()), // Plosive?
            "гъ" => Consonant::new(Uvular, Fricative, Voiced, false, s.to_owned()),
            // Fricatives Voiced Labialized
            "гъу" => Consonant::new(Uvular, Fricative, Voiced, true, s.to_owned()),
            // Fricatives Ejective
            "фI" => Consonant::new(Labial, Fricative, Ejective, false, s.to_owned()),
            "лI" => Consonant::new(Lateral, Fricative, Ejective, false, s.to_owned()),
            "щI" => Consonant::new(Palatal, Fricative, Ejective, false, s.to_owned()),
            // Trills
            "р" => Consonant::new(Alveolar, Trill, Voiced, false, s.to_owned()),
            "й" => Consonant::new(Palatal, Approximant, Voiced, false, s.to_owned()),
            // Consider actually using "w" for this, because у can also be a combi.
            "у" => Consonant::new(Labial, Approximant, Voiced, false, s.to_owned()), // labialized ?

            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Display for Consonant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Manner::*;
        use Place::*;
        use Voiceness::*;
        let x = match (self.place, self.manner, self.voiceness, self.is_labialized) {
            // Plosives
            (Labial, Nasal, Voiced, false) => "н".to_owned(),
            (Alveolar, Nasal, Voiceless, false) => "м".to_owned(),

            (Labial, Plosive, Voiceless, false) => "п".to_owned(),
            (Labial, Plosive, Voiced, false) => "б".to_owned(),
            (Labial, Plosive, Ejective, false) => "пI".to_owned(),

            (Alveolar, Plosive, Voiceless, false) => "т".to_owned(),
            (Alveolar, Plosive, Voiced, false) => "д".to_owned(),
            (Alveolar, Plosive, Ejective, false) => "тI".to_owned(),

            (Velar, Plosive, Voiceless, false) => "к".to_owned(),

            (Uvular, Plosive, Voiceless, false) => "къ".to_owned(),
            (Glottal, Plosive, Voiceless, false) => "I".to_owned(),

            (Uvular, Plosive, Voiceless, true) => "къу".to_owned(),
            (Glottal, Plosive, Voiceless, true) => "Iу".to_owned(),

            // Fricatives
            (Alveolar, Affricative, Voiceless, false) => "ц".to_owned(),
            (Alveolar, Affricative, Voiced, false) => "дз".to_owned(),
            (Alveolar, Affricative, Ejective, false) => "цI".to_owned(),

            (PostAlveolar, Affricative, Voiceless, false) => "ч".to_owned(),
            (PostAlveolar, Affricative, Voiced, false) => "дж".to_owned(),
            (PostAlveolar, Affricative, Ejective, false) => "кI".to_owned(),
            (Uvular, Affricative, Voiceless, false) => "кхъ".to_owned(),

            (Uvular, Affricative, Voiceless, true) => "кхъу".to_owned(),



            (Labial, Fricative, Voiceless, false) => "ф".to_owned(),
            (Labial, Fricative, Voiced, false) => "в".to_owned(),
            (Labial, Fricative, Ejective, false) => "фI".to_owned(),

            (Alveolar, Fricative, Voiceless, false) => "с".to_owned(),
            (Alveolar, Fricative, Voiced, false) => "з".to_owned(),

            (Lateral, Fricative, Voiceless, false) => "лъ".to_owned(),
            (Lateral, Fricative, Voiced, false) => "л".to_owned(),
            (Lateral, Fricative, Ejective, false) => "лI".to_owned(),

            (PostAlveolar, Fricative, Voiceless, false) => "ш".to_owned(),
            (PostAlveolar, Fricative, Voiced, false) => "ж".to_owned(),

            (Palatal, Fricative, Voiceless, false) => "щ".to_owned(),
            (Palatal, Fricative, Voiced, false) => "жь".to_owned(),
            (Palatal, Fricative, Ejective, false) => "щI".to_owned(),

            (Velar, Fricative, Voiceless, false) => "х".to_owned(),
            (Velar, Fricative, Voiced, false) => "г".to_owned(),
            (Uvular, Fricative, Voiceless, false) => "хъ".to_owned(),
            (Uvular, Fricative, Voiced, false) => "гъ".to_owned(),
            (Pharyngeal, Fricative, Voiceless, false) => "хь".to_owned(),

            (Velar, Fricative, Voiceless, true) => "ху".to_owned(),
            (Uvular, Fricative, Voiceless, true) => "хъу".to_owned(),
            (Uvular, Fricative, Voiced, true) => "гьу".to_owned(),

            (Palatal, Approximant, Voiced, false) => "й".to_owned(),
            (Labial, Approximant, Voiced, false) => "у".to_owned(),
            (Alveolar, Trill, Voiced, false) => "р".to_owned(),


            // 
            (Velar, Plosive, Voiced, false) => unreachable!("kabardian doesn't have a voiced velar plosive, did you mean voiced velar fricative?"),
            (Labial, _, _, true) => unreachable!(""),
            (Alveolar, _, _, true) => unreachable!(""),
            (PostAlveolar, _, _, true) => unreachable!(""),
            (Pharyngeal, _, _, true) => unreachable!(""),
            // (Ignore, _, _, _) => panic!(""),
            // (_, Manner::Ignore, _, _) => panic!(""),

            x => unimplemented!("{:?}", x),
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Labial,
    Alveolar,
    Lateral,
    PostAlveolar,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manner {
    Nasal,
    Plosive,
    Affricative,
    Fricative,
    Approximant,
    Trill,
}

fn first_of_combi(c: &char) -> Result<char, String> {
    match c {
        'я' | 'е' | 'и' => Ok('й'),
        'о' | 'у' => Ok('у'),
        _ => Err(format!("{} is not a combi", c)),
    }
}
fn last_of_combi(c: &char) -> Result<char, String> {
    match c {
        'я' => Ok('а'),
        'е' | 'о' => Ok('э'),
        'и' | 'у' => Ok('ы'),
        _ => Err(format!("{} is not a combi", c)),
    }
}
fn split_combi(combi: &char) -> Result<(Consonant, Vowel), String> {
    if !is_char_combi(combi) {
        return Err(format!("{} is not a combi", combi));
    }
    let first = first_of_combi(combi).expect("Parameter is a combi.");
    let last = last_of_combi(combi).expect("Parameter is a combi.");

    let c = Consonant::try_from(first.to_string()).expect("This must be a consonant.");
    let v = Vowel::try_from(last.to_string()).expect("This must be a vowel.");
    Ok((c, v))
}
fn combine_to_combi(c_0: &String, c_1: &String) -> Option<char> {
    match (&c_0.as_ref(), &c_1.as_ref()) {
        (&"й", &"э") => Some('e'),
        (&"й", &"ы") => Some('и'),
        (&"й", &"а") => Some('я'),
        (&"у", &"э") => Some('о'),
        (&"у", &"ы") => Some('у'),
        _ => None,
    }
}
fn is_char_combi(c: &char) -> bool {
    match c {
        'я' | 'е' | 'и' | 'о' | 'у' => true,
        _ => false,
    }
}
fn is_char_vowel(c: &char) -> bool {
    match c {
        'а' | 'э' | 'ы' => true,
        _ => false,
    }
}
fn is_char_diacritic(c: &char) -> bool {
    match c {
        'ь' | 'ъ' => true,
        'I' | 'у' => true,
        _ => false,
    }
}
fn is_char_consonant(c: &char) -> bool {
    match c {
        'м' | 'н' => true,
        'I' => true,
        'п' | 'б' | 'т' | 'д' | 'к' | 'г' => true,
        'ф' | 'в' | 'с' | 'з' | 'ш' | 'щ' | 'х' | 'ж' | 'р' | 'л' | 'й' | 'ч' | 'ц' => {
            true
        }
        _ => false,
    }
}

/// Parses a string into a vector of letters.
///
/// This is especially useful since many letters in Kabardian are di-, tri-, or even quadrigraphs.
pub fn parse(s: &str) -> Result<Vec<Letter>, String> {
    let mut letters = Vec::new();
    let chars = s.chars().collect::<Vec<char>>();
    let possible_charaters = [
        'ь', 'ъ', //
        'а', 'э', 'ы', //
        'я', 'е', 'и', 'о', 'у', 'ю', //
        'м', 'н', //
        'п', 'б', 'т', 'д', 'к', 'г', 'I', //
        'ф', 'в', 'с', 'з', 'ш', 'щ', 'х', 'ж', //
        'ч', 'ц', //
        'р', 'л', 'й', //
    ];
    for c in &chars {
        if !possible_charaters.contains(c) {
            return Err(format!("invalid character: {}", c));
        }
    }

    let mut i = 0;
    while i < chars.len() {
        let ch = match &chars.get(i) {
            Some(c) => *c,
            None => break,
        };

        let letter = match ch {
            vowel if is_char_vowel(vowel) => Letter {
                kind: LetterKind::Vowel(
                    Vowel::try_from(vowel.to_string())
                        .expect("Already checked whether the character is a vowel."),
                ),
                base: vowel.to_string(),
            },
            combi if is_char_combi(combi) => {
                let (c, v) = split_combi(combi).expect("Parameter is a combi.");
                Letter {
                    kind: LetterKind::Combi(c, v),
                    base: combi.to_string(),
                }
            }
            consonant if is_char_consonant(consonant) => {
                #[derive(PartialEq)]
                enum Deco {
                    Base,
                    Palotshka,
                    MagkiyZnak,
                    TverdyyZnak,
                }
                let can_palochka = ['п', 'т', 'ф', 'щ', 'к', 'х', 'ц', 'л'];
                let can_labialized = ['к', 'г', 'х', 'I'];
                let can_tverdyj_znak = ['к', 'х', 'г', 'л'];
                let can_magkiy_znak = ['х', 'ж'];
                let simple_cons = ['м', 'н', 'б', 'ч', 'с', 'з', 'ш', 'в', 'р'];

                let mut consonant_str: Vec<char> = Vec::new();
                consonant_str.push(*consonant);

                // attribs of the current letter
                let base_char = consonant_str[0];
                let mut has_labial = false;
                let mut deco = Deco::Base;

                'bp: loop {
                    if simple_cons.contains(&base_char) {
                        break 'bp;
                    }
                    let next_letter = chars.get(i + 1);
                    if next_letter.map(is_char_diacritic).unwrap_or(false) {}
                    if next_letter.map(is_char_vowel).unwrap_or(false) {
                        break 'bp;
                    }

                    match &next_letter {
                        Some(c)
                            if can_palochka.contains(&base_char)
                                && deco == Deco::Base
                                && !has_labial
                                && *c == &'I' =>
                        {
                            consonant_str.push(**c);
                            i += 1;
                            deco = Deco::Palotshka;
                        }
                        Some(c)
                            if can_labialized.contains(&base_char)
                                && !has_labial
                                && deco != Deco::MagkiyZnak
                                && *c == &'у' =>
                        {
                            consonant_str.push(**c);
                            i += 1;
                            has_labial = true;
                        }
                        Some(c)
                            if can_tverdyj_znak.contains(&base_char)
                                && deco == Deco::Base
                                && !has_labial
                                && *c == &'ъ' =>
                        {
                            consonant_str.push(**c);
                            i += 1;
                            deco = Deco::TverdyyZnak;
                        }
                        Some(c)
                            if can_magkiy_znak.contains(&base_char)
                                && deco == Deco::Base
                                && !has_labial
                                && *c == &'ь' =>
                        {
                            consonant_str.push(**c);
                            i += 1;
                            deco = Deco::MagkiyZnak;
                        }
                        Some(c @ 'з') | Some(c @ 'ж') if base_char == 'д' => {
                            consonant_str.push(**c);
                            i += 1;
                        }
                        Some(c @ 'х')
                            if base_char == 'к'
                                && chars.get(i + 2).map(|x| x == &'ъ').unwrap_or(false) =>
                        {
                            consonant_str.push(**c);
                            i += 1;
                        }
                        _ => break 'bp,
                    }
                }
                let consonant = Consonant::try_from(consonant_str.iter().collect::<String>())
                    .expect("If this panics, there is a bug in the code");
                Letter {
                    kind: LetterKind::Consonant(consonant.clone()),
                    base: consonant.base.clone(),
                }
            }
            x => unreachable!("This should be unreachable {:?}", x),
        };
        letters.push(letter);
        i += 1;
    }

    Ok(letters)
}
