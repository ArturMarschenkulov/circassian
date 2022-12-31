/// This module handles the orthography of the language.
/// The goal is to convert every letter into a unit with useful meta-data.
///
/// The main struct is `Letter`, which contains the letter itself, and a `LetterKind`.

#[derive(Debug, Clone, PartialEq)]
pub struct Letter {
    pub kind: LetterKind,
    base: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LetterKind {
    /// Represents a consonant letter.
    Consonant(Consonant),
    /// Represents a vowel letter.
    Vowel(Vowel),
    /// Represents special letters which are a combination of consonant and vowel
    /// Those are 'я', 'е', 'и', 'у', 'о'
    /// 'я' = 'йа',
    /// 'е' = 'йэ',
    /// 'и' = 'йы',
    /// 'у' = 'уы',
    /// 'о' = 'уэ', (actually it's rather'эу')
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

    pub fn is_consonant_voiceness(&self, voiceness: Voiceness) -> bool {
        match &self.kind {
            LetterKind::Consonant(c) => c.voiceness == voiceness,
            LetterKind::Vowel(..) => false,
            LetterKind::Combi(..) => false,
        }
    }
    pub fn is_consonant_manner(&self, manner: Manner) -> bool {
        match &self.kind {
            LetterKind::Consonant(c) => c.manner == manner,
            LetterKind::Vowel(..) => false,
            LetterKind::Combi(..) => false,
        }
    }
    pub fn is_consonant_place(&self, place: Place) -> bool {
        match &self.kind {
            LetterKind::Consonant(c) => c.place == place,
            LetterKind::Vowel(..) => false,
            LetterKind::Combi(..) => false,
        }
    }
    pub fn is_consonant_manner_place(&self, manner: Manner, place: Place) -> bool {
        self.is_consonant_manner(manner) && self.is_consonant_place(place)
    }
    pub fn is_nasal(&self) -> bool {
        self.is_consonant_manner(Manner::Nasal)
    }
    pub fn is_trill(&self) -> bool {
        self.is_consonant_manner(Manner::Trill)
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
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = match &self.kind {
            LetterKind::Consonant(c) => c.to_string(),
            LetterKind::Vowel(v) => v.to_string(),
            LetterKind::Combi(c, v) => {
                let x = match (&c.to_string().as_ref(), &v.to_string().as_ref()) {
                    (&"й", &"э") => "е".to_owned(),
                    (&"й", &"ы") => "и".to_owned(),
                    (&"й", &"а") => "я".to_owned(),
                    (&"у", &"ы") => "у".to_owned(),
                    (&"у", &"э") => "о".to_owned(),
                    _ => unreachable!(),
                };
                x
            }
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vowel {
    kind: VowelKind,
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
    fn from_string(s: String) -> Option<Vowel> {
        match s.as_str() {
            "а" => Some(Vowel::new(VowelKind::AA)),
            "э" => Some(Vowel::new(VowelKind::A)),
            "ы" => Some(Vowel::new(VowelKind::Y)),
            _ => None,
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
enum VowelKind {
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
    is_labialized: bool,
    pub base: String,
}
impl Consonant {
    fn new(
        place: Place,
        manner: Manner,
        voiceness: Voiceness,
        is_labialized: bool,
        base: String,
    ) -> Option<Self> {
        Some(Consonant {
            place,
            
            manner,
            voiceness,
            is_labialized,
            base,
        })
    }
    /// Returns `true` if the consonant is a labial plosive, i.e. 'п' or 'б'.
    pub fn is_labial_plosive(&self) -> bool {
        self.place == Place::Labial && self.manner == Manner::Plosive
    }
    /// Returns `true` if the consonant is a velar, uvular or pharyngeal plosive, aka it needs an 'ы' before 'у'.
    pub fn needs_epenthetic_y(&self) -> bool {
        use Place::*;
        [Velar, Uvular, Glottal].contains(&self.place)
    }
    fn from_string(s: &String) -> Option<Consonant> {
        match s.as_str() {
            // Nasals
            "м" => Consonant::new(
                Place::Labial,
                Manner::Nasal,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "н" => Consonant::new(
                Place::Alveolar,
                Manner::Nasal,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            // Plosives Unvoiced
            "п" => Consonant::new(
                Place::Labial,
                Manner::Plosive,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "т" => Consonant::new(
                Place::Alveolar,
                Manner::Plosive,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "к" => Consonant::new(
                Place::Velar,
                Manner::Plosive,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "къ" => Consonant::new(
                Place::Uvular,
                Manner::Plosive,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "I" => Consonant::new(
                Place::Glottal,
                Manner::Plosive,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "Iу" => Consonant::new(
                Place::Glottal,
                Manner::Plosive,
                Voiceness::Voiceless,
                true,
                s.to_owned(),
            ),
            // Plosives Unvoiced Labialized
            "ку" => Consonant::new(
                Place::Velar,
                Manner::Plosive,
                Voiceness::Voiceless,
                true,
                s.to_owned(),
            ),
            "кьу" => Consonant::new(
                Place::Uvular,
                Manner::Plosive,
                Voiceness::Voiceless,
                true,
                s.to_owned(),
            ),

            // Plosives Voiced
            "б" => Consonant::new(
                Place::Labial,
                Manner::Plosive,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "д" => Consonant::new(
                Place::Alveolar,
                Manner::Plosive,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),

            // Plosives Voiced Labialized
            "гу" => Consonant::new(
                Place::Velar,
                Manner::Plosive,
                Voiceness::Voiced,
                true,
                s.to_owned(),
            ),
            // Plosives Ejective
            "пI" => Consonant::new(
                Place::Labial,
                Manner::Plosive,
                Voiceness::Ejective,
                false,
                s.to_owned(),
            ),
            "тI" => Consonant::new(
                Place::Alveolar,
                Manner::Plosive,
                Voiceness::Ejective,
                false,
                s.to_owned(),
            ),
            // Plosives Ejective Labialized
            "кIу" => Consonant::new(
                Place::Velar,
                Manner::Plosive,
                Voiceness::Ejective,
                true,
                s.to_owned(),
            ),
            // Affricates Unvoiced
            "ц" => Consonant::new(
                Place::Alveolar,
                Manner::Affricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "ч" => Consonant::new(
                Place::PostAlveolar,
                Manner::Affricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "кхъ" => Consonant::new(
                Place::Uvular,
                Manner::Affricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            // Affricates Unvoiced Labialized
            "кхъу" => Consonant::new(
                Place::Uvular,
                Manner::Affricative,
                Voiceness::Voiceless,
                true,
                s.to_owned(),
            ),
            // Affricates Voiced
            "дз" => Consonant::new(
                Place::Alveolar,
                Manner::Affricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "дж" => Consonant::new(
                Place::PostAlveolar,
                Manner::Affricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            // Affricates Ejecitive
            "цI" => Consonant::new(
                Place::Alveolar,
                Manner::Affricative,
                Voiceness::Ejective,
                false,
                s.to_owned(),
            ),
            "кI" => Consonant::new(
                Place::PostAlveolar,
                Manner::Affricative,
                Voiceness::Ejective,
                false,
                s.to_owned(),
            ),
            // Fricatives Unvoiced
            "ф" => Consonant::new(
                Place::Labial,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "с" => Consonant::new(
                Place::Alveolar,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "лъ" => Consonant::new(
                Place::Lateral,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "ш" => Consonant::new(
                Place::PostAlveolar,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "щ" => Consonant::new(
                Place::Palatal,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "х" => Consonant::new(
                Place::Velar,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "хъ" => Consonant::new(
                Place::Uvular,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            "хь" => Consonant::new(
                Place::Pharyngeal,
                Manner::Fricative,
                Voiceness::Voiceless,
                false,
                s.to_owned(),
            ),
            // Fricatives Unvoiced Labialized
            "ху" => Consonant::new(
                Place::Velar,
                Manner::Fricative,
                Voiceness::Voiceless,
                true,
                s.to_owned(),
            ),
            "хъу" => Consonant::new(
                Place::Uvular,
                Manner::Fricative,
                Voiceness::Voiceless,
                true,
                s.to_owned(),
            ),
            // Fricatives Voiced
            "в" => Consonant::new(
                Place::Labial,
                Manner::Fricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "з" => Consonant::new(
                Place::Alveolar,
                Manner::Fricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "л" => Consonant::new(
                Place::Lateral,
                Manner::Fricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "ж" => Consonant::new(
                Place::PostAlveolar,
                Manner::Fricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "жь" => Consonant::new(
                Place::Palatal,
                Manner::Fricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "г" => Consonant::new(
                Place::Velar,
                Manner::Fricative, // Plosive?
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "гъ" => Consonant::new(
                Place::Uvular,
                Manner::Fricative,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            // Fricatives Voiced Labialized
            "гъу" => Consonant::new(
                Place::Uvular,
                Manner::Fricative,
                Voiceness::Voiced,
                true,
                s.to_owned(),
            ),
            // Fricatives Ejective
            "фI" => Consonant::new(
                Place::Labial,
                Manner::Fricative,
                Voiceness::Ejective,
                false,
                s.to_owned(),
            ),
            "лI" => Consonant::new(
                Place::Lateral,
                Manner::Fricative,
                Voiceness::Ejective,
                false,
                s.to_owned(),
            ),
            "щI" => Consonant::new(
                Place::Palatal,
                Manner::Fricative,
                Voiceness::Ejective,
                false,
                s.to_owned(),
            ),
            // Trills
            "р" => Consonant::new(
                Place::Alveolar,
                Manner::Trill,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            "й" => Consonant::new(
                Place::Palatal,
                Manner::Approximant,
                Voiceness::Voiced,
                false,
                s.to_owned(),
            ),
            // Consider actually using "w" for this, because у can also be a combi.
            "у" => Consonant::new(
                Place::Labial,
                Manner::Approximant,
                Voiceness::Voiced,
                false, // labialized ?
                s.to_owned(),
            ),

            _ => unimplemented!(),
        }
    }
}


impl std::fmt::Display for Consonant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = match (self.place, self.manner, self.voiceness, self.is_labialized) {
            // Plosives
            (Place::Labial, Manner::Nasal, Voiceness::Voiced, false) => "н".to_owned(),
            (Place::Alveolar, Manner::Nasal, Voiceness::Voiceless, false) => "м".to_owned(),

            (Place::Labial, Manner::Plosive, Voiceness::Voiceless, false) => "п".to_owned(),
            (Place::Labial, Manner::Plosive, Voiceness::Voiced, false) => "б".to_owned(),
            (Place::Labial, Manner::Plosive, Voiceness::Ejective, false) => "пI".to_owned(),

            (Place::Alveolar, Manner::Plosive, Voiceness::Voiceless, false) => "т".to_owned(),
            (Place::Alveolar, Manner::Plosive, Voiceness::Voiced, false) => "д".to_owned(),
            (Place::Alveolar, Manner::Plosive, Voiceness::Ejective, false) => "тI".to_owned(),

            (Place::Velar, Manner::Plosive, Voiceness::Voiceless, false) => "к".to_owned(),

            (Place::Uvular, Manner::Plosive, Voiceness::Voiceless, false) => "къ".to_owned(),
            (Place::Glottal, Manner::Plosive, Voiceness::Voiceless, false) => "I".to_owned(),

            (Place::Uvular, Manner::Plosive, Voiceness::Voiceless, true) => "къу".to_owned(),
            (Place::Glottal, Manner::Plosive, Voiceness::Voiceless, true) => "Iу".to_owned(),

            // Fricatives
            (Place::Alveolar, Manner::Affricative, Voiceness::Voiceless, false) => "ц".to_owned(),
            (Place::Alveolar, Manner::Affricative, Voiceness::Voiced, false) => "дз".to_owned(),
            (Place::Alveolar, Manner::Affricative, Voiceness::Ejective, false) => "цI".to_owned(),
            
            (Place::PostAlveolar, Manner::Affricative, Voiceness::Voiceless, false) => "ч".to_owned(),
            (Place::PostAlveolar, Manner::Affricative, Voiceness::Voiced, false) => "дж".to_owned(),
            (Place::PostAlveolar, Manner::Affricative, Voiceness::Ejective, false) => "кI".to_owned(),
            (Place::Uvular, Manner::Affricative, Voiceness::Voiceless, false) => "кхъ".to_owned(),

            (Place::Uvular, Manner::Affricative, Voiceness::Voiceless, true) => "кхъу".to_owned(),



            (Place::Labial, Manner::Fricative, Voiceness::Voiceless, false) => "ф".to_owned(),
            (Place::Labial, Manner::Fricative, Voiceness::Voiced, false) => "в".to_owned(),
            (Place::Labial, Manner::Fricative, Voiceness::Ejective, false) => "фI".to_owned(),

            (Place::Alveolar, Manner::Fricative, Voiceness::Voiceless, false) => "с".to_owned(),
            (Place::Alveolar, Manner::Fricative, Voiceness::Voiced, false) => "з".to_owned(),

            (Place::Lateral, Manner::Fricative, Voiceness::Voiceless, false) => "лъ".to_owned(),
            (Place::Lateral, Manner::Fricative, Voiceness::Voiced, false) => "л".to_owned(),
            (Place::Lateral, Manner::Fricative, Voiceness::Ejective, false) => "лI".to_owned(),

            (Place::PostAlveolar, Manner::Fricative, Voiceness::Voiceless, false) => "ш".to_owned(),
            (Place::PostAlveolar, Manner::Fricative, Voiceness::Voiced, false) => "ж".to_owned(),

            (Place::Palatal, Manner::Fricative, Voiceness::Voiceless, false) => "щ".to_owned(),
            (Place::Palatal, Manner::Fricative, Voiceness::Voiced, false) => "жь".to_owned(),
            (Place::Palatal, Manner::Fricative, Voiceness::Ejective, false) => "щI".to_owned(),

            (Place::Velar, Manner::Fricative, Voiceness::Voiceless, false) => "х".to_owned(),
            (Place::Velar, Manner::Fricative, Voiceness::Voiced, false) => "г".to_owned(),
            (Place::Uvular, Manner::Fricative, Voiceness::Voiceless, false) => "хъ".to_owned(),
            (Place::Uvular, Manner::Fricative, Voiceness::Voiced, false) => "гъ".to_owned(),
            (Place::Pharyngeal, Manner::Fricative, Voiceness::Voiceless, false) => "хь".to_owned(),
            

            (Place::Velar, Manner::Fricative, Voiceness::Voiceless, true) => "ху".to_owned(),
            (Place::Uvular, Manner::Fricative, Voiceness::Voiceless, true) => "хъу".to_owned(),
            (Place::Uvular, Manner::Fricative, Voiceness::Voiced, true) => "гьу".to_owned(),

            (Place::Palatal, Manner::Approximant, Voiceness::Voiced, false) => "й".to_owned(),
            (Place::Labial, Manner::Approximant, Voiceness::Voiced, false) => "у".to_owned(),
            (Place::Alveolar, Manner::Trill, Voiceness::Voiced, false) => "р".to_owned(),


            // 
            (Place::Velar, Manner::Plosive, Voiceness::Voiced, false) => unreachable!("kabardian doesn't have a voiced velar plosive, did you mean voiced velar fricative?"),
            (Place::Labial, _, _, true) => unreachable!(""),
            (Place::Alveolar, _, _, true) => unreachable!(""),
            (Place::PostAlveolar, _, _, true) => unreachable!(""),
            (Place::Pharyngeal, _, _, true) => unreachable!(""),
            // (Place::Ignore, _, _, _) => panic!(""),
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
pub fn parse(s: &str) -> Vec<Letter> {
    let mut letters = Vec::new();
    let chars = s.chars().collect::<Vec<char>>();

    let mut i = 0;
    while i < chars.len() {
        let ch = match &chars.get(i) {
            Some(c) => *c,
            None => break,
        };

        let letter = match ch {
            vowel if is_char_vowel(vowel) => Letter {
                kind: LetterKind::Vowel(Vowel::from_string(vowel.to_string()).unwrap()),
                base: vowel.to_string(),
            },
            combi if is_char_combi(combi) => {
                let c = match combi {
                    'я' | 'е' | 'и' => 'й',
                    'о' | 'у' => 'у',
                    _ => unreachable!(),
                };
                let v = match combi {
                    'я' => 'а',
                    'е' | 'о' => 'э',
                    'и' | 'у' => 'ы',
                    _ => unreachable!(),
                };

                let consonant = Consonant::from_string(&c.to_string()).unwrap();
                let vowel = Vowel::from_string(v.to_string()).unwrap();
                Letter {
                    kind: LetterKind::Combi(consonant, vowel),
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
                let can_labialized = ['к', 'х', 'I'];
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
                        // println!("{:?} is a simply consonant, it can't be modified, thus we are breaking", consonant_str);
                        break 'bp;
                    }
                    let next_letter = chars.get(i + 1);
                    // println!("The next letter is {:?}", next_letter);
                    if next_letter.map(is_char_diacritic).unwrap_or(false) {
                        // println!("The next letter is a diacritic {:?}", next_letter);
                    }
                    if next_letter.map(is_char_vowel).unwrap_or(false) {
                        // println!("The next letter is a vowel {:?}", next_letter);
                        break 'bp;
                    }

                    match &next_letter {
                        pal @ Some('I')
                            if can_palochka.contains(&base_char)
                                && deco == Deco::Base
                                && !has_labial =>
                        {
                            consonant_str.push(*pal.unwrap());
                            i += 1;
                            deco = Deco::Palotshka;
                        }
                        lab @ Some('у')
                            if can_labialized.contains(&base_char)
                                && !has_labial
                                && deco != Deco::MagkiyZnak =>
                        {
                            consonant_str.push(*lab.unwrap());
                            i += 1;
                            has_labial = true;
                        }
                        tz @ Some('ъ')
                            if can_tverdyj_znak.contains(&base_char)
                                && deco == Deco::Base
                                && !has_labial =>
                        {
                            consonant_str.push(*tz.unwrap());
                            i += 1;
                            deco = Deco::TverdyyZnak;
                        }
                        mz @ Some('ь')
                            if can_magkiy_znak.contains(&base_char)
                                && deco == Deco::Base
                                && !has_labial =>
                        {
                            consonant_str.push(*mz.unwrap());
                            i += 1;
                            deco = Deco::MagkiyZnak;
                        }
                        aff @ (Some('з') | Some('ж')) if base_char == 'д' => {
                            consonant_str.push(*aff.unwrap());
                            i += 1;
                        }
                        aff @ Some('х')
                            if base_char == 'к'
                                && chars.get(i + 2).map(|x| x == &'ъ').unwrap_or(false) =>
                        {
                            consonant_str.push(*aff.unwrap());
                            i += 1;
                        }
                        _ => break 'bp,
                    }
                }
                let s = consonant_str.iter().collect::<String>();
                let consonant = Consonant::from_string(&s).unwrap();
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

    letters
}
