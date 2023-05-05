use crate::morpho::{self, Preverb, Transitivity};
use crate::ortho;

/// It's basically there to treat stems ending on у which labializes the consonants before it.
/// This also indicates an implicit ы.
pub fn treat_thematic_vowel(tv: &ThematicVowel, vs: &VerbStem) -> String {
    match (&tv, &vs.last_consonant) {
        (ThematicVowel::Y, LastConsonant::Labial) => "",
        (ThematicVowel::Y, _) => "ы",
        _ => "э",
    }
    .to_owned()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VowelKind {
    With,
    Without,
    Alternating,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum ConsonantKind {
//     /// Refers to unvoiced consonants.
//     /// Can only be in the beginning of transitive verbs.
//     /// It makes the preceding consonant unvoiced.
//     /// Comes from 'д'.
//     Unvoiced,
//     /// Refers to voiced consonants.
//     /// Can only be in the beginning of transitive verbs.
//     /// It makes the preceding consonant voiced.
//     /// Comes from 'жъ'.
//     Voiced,
//     /// Refers to velar consonants.
//     /// Can only be in the end.
//     /// Forces an insertion of ы before a у, to differentiate between labialized consonants къу vs къыу
//     /// Comes from 'т'.
//     Velar,
//     /// Refers to labialized consonants.
//     /// Can only be in the end.
//     /// There can't be an ы behind it, as it's already implicit. гуыр -> гъур
//     /// Comes from 'л'.
//     Labialized,
//     /// Refers to consonants that are neither voiced nor unvoiced.
//     /// Can only be in the end. Intransitive verbs can also have it at the beginning, because there voiceness doesn't matter.
//     ///
//     /// Comes from 'д'.
//     Ordinary,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThematicVowel {
    A,
    Y,
}

impl ThematicVowel {
    fn from(vowel: &ortho::Vowel) -> Self {
        match vowel.kind {
            ortho::VowelKind::A => ThematicVowel::A,
            ortho::VowelKind::Y => ThematicVowel::Y,
            _ => unreachable!("The vowel {:?} is not a thematic vowel.", vowel),
        }
    }
}
impl std::fmt::Display for ThematicVowel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ThematicVowel::A => write!(f, "э"),
            ThematicVowel::Y => write!(f, "ы"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirstConsonant {
    Unvoiced, // 'д'
    Voiced,   // 'жь'
    Wy,       // 'у'
              // Yy, // 'й'
}

impl FirstConsonant {
    fn from(consonant: &ortho::Consonant) -> Self {
        match (consonant.place, consonant.manner, consonant.voiceness) {
            (ortho::Place::Labial, ortho::Manner::Approximant, _) => FirstConsonant::Wy,
            (_, _, ortho::Voiceness::Voiced) => FirstConsonant::Voiced,
            _ => FirstConsonant::Unvoiced,
            // _ => panic!("The consonant {:?} is not a first consonant.", consonant),
        }
    }
    pub fn to_voiceness(&self) -> ortho::Voiceness {
        match self {
            FirstConsonant::Unvoiced => ortho::Voiceness::Voiceless,
            FirstConsonant::Voiced => ortho::Voiceness::Voiced,
            FirstConsonant::Wy => ortho::Voiceness::Voiced,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LastConsonant {
    Ordinary, // 'д'
    Velar,    // 'т'
    Labial,   // 'л'
    Yy,       // 'й'
}

impl LastConsonant {
    fn from(consonant: &ortho::Consonant) -> Self {
        match (consonant.place, consonant.manner, consonant.is_labialized) {
            (_, _, true) => LastConsonant::Labial,
            (ortho::Place::Velar, _, _) => LastConsonant::Velar,
            (ortho::Place::Palatal, ortho::Manner::Approximant, _) => LastConsonant::Yy,
            _ => LastConsonant::Ordinary,
            // _ => panic!("The consonant {:?} is not a last consonant.", consonant),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbStem {
    pub first_consonant: Option<FirstConsonant>,
    pub vowel: VowelKind,
    pub last_consonant: LastConsonant,
    pub is_alternating: bool,
    pub thematic_vowel: ThematicVowel,
    pub string: String,
}

impl VerbStem {
    pub fn new_from_str(s: &str, transitivity: morpho::Transitivity) -> Self {
        let letters = ortho::parse(s).unwrap();
        assert!(!letters.is_empty(), "The verb stem can't be empty.");
        println!("letters: {:#?}", letters);

        let vowel = if letters.len() == 1 {
            VowelKind::Without
        } else {
            VowelKind::With
        };

        let last_letter = letters.last().unwrap();
        let first_letter = letters.first().unwrap();

        let thematic_vowel = if let ortho::LetterKind::Vowel(vowel) = &last_letter.kind {
            ThematicVowel::from(vowel)
        } else {
            ThematicVowel::Y
        };

        let last_consonant = letters
            .iter()
            .rev()
            .find(|l| l.is_consonant())
            .map(|l| {
                if let ortho::LetterKind::Consonant(consonant) = &l.kind {
                    LastConsonant::from(consonant)
                } else {
                    panic!("The letter {:?} is not a consonant.", l);
                }
            })
            .unwrap();

        // The first consonant is only relevant for transitive verbs.
        let first_consonant = match transitivity {
            Transitivity::Intransitive => None,
            _ => {
                if let ortho::LetterKind::Consonant(consonant) = &first_letter.kind {
                    Some(FirstConsonant::from(consonant))
                } else {
                    panic!("The letter {:?} is not a consonant.", first_letter);
                }
            }
        };

        // в / вы

        Self {
            first_consonant,
            vowel,
            last_consonant,
            is_alternating: false,
            thematic_vowel,
            string: s.to_owned(),
        }
    }
}

// /// Here is the information stored about the verb stem.
// /// It is extracted from the template string.
// /// In the Kabardian language itself, all stems are mostly treated the same, however because of the orthographical system
// /// there are some difference how those stems are treated.
// #[derive(Debug, Clone, PartialEq)]
// pub struct VerbStem {
//     pub first_consonant: ConsonantKind,
//     vowel: VowelKind,
//     last_consonant: ConsonantKind,
//     pub thematic_vowel: ThematicVowel,
//     pub string: String,
// }

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

fn extract_transitivity(s: &str) -> Result<Transitivity, String> {
    match s {
        "лъэмыӏ" => Ok(Transitivity::Intransitive),
        "лъэӏ" => Ok(Transitivity::Transitive),
        _ => Err(format!(
            "The transitivity must be either лъэмыӏ or лъэӏ, but it is {}.",
            s
        )),
    }
}
fn extract_thematic_vowel(s: &str) -> Result<ThematicVowel, String> {
    match s {
        "э" => Ok(ThematicVowel::A),
        "ы" => Ok(ThematicVowel::Y),

fn extract_preverb(s: &str) -> Option<Preverb> {
    match s {
        "0" => None,
        _ => Some(Preverb::new(&s.to_owned())),
    }
}

fn extract_root(s: &str) -> (Option<FirstConsonant>, VowelKind, LastConsonant, bool) {
    //  let segments = s.split('-').collect::<Vec<&str>>();
    let root = <&str>::clone(&s).to_string();
    // the root string can only contain either 2, 3 or 5 characters.
    assert!(
        [2, 3, 4].contains(&root.chars().count()),
        "The root string is not 2, 3 or 5 characters long {} {}",
        &root.len(),
        root
    );

    // parsing first consonant
    let c = &root.chars().next().unwrap();
    let (root, fc) = if !['б', '0'].contains(c) {
        // this means it's transitive.
        let c = &root.chars().next().unwrap();
        let fc = match c {
            'д' => Some(FirstConsonant::Unvoiced),
            'у' => Some(FirstConsonant::Wy),
            'ж' => Some(FirstConsonant::Voiced),
            _ => panic!("The first consonant is not one of the allowed consonants"),
        };

        let root = if c == &'ж' {
            root.replacen("жь", "", 1)
        } else {
            root.replacen(*c, "", 1)
        };
        (root, fc)
    } else {
        // this means it's intransitive, because such verbs don't have phonological processes where the first letter matters.
        (root, None)
    };

    let c = &root.chars().next().unwrap();
    let (root, v) = if ['б', '0'].contains(c) {
        let v = match c {
            'б' => VowelKind::With,
            '0' => VowelKind::Without,
            _ => panic!("The first consonant is not one of the allowed consonants"),
        };
        let root = root.replacen(*c, "", 1);
        (root, v)
    } else {
        panic!("")
    };

    let c = &root.chars().next().unwrap();
    let (root, lc) = if !['б', '0'].contains(c) {
        // this means it's transitive.
        let c = &root.chars().next().unwrap();
        let fc = match c {
            'д' => LastConsonant::Ordinary,
            'т' => LastConsonant::Velar,
            'л' => LastConsonant::Labial,
            _ => panic!(""),
        };

        let root = root.replacen(*c, "", 1);
        (root, fc)
    } else {
        panic!("")
    };

    let alternating = root.ends_with("эа");
    (fc, v, lc, alternating)
}

#[derive(Debug, Clone)]
pub struct TemplateDesc {
    pub transitivity: Transitivity,
    pub preverb: Option<Preverb>,
    pub stem: VerbStem,
    pub original_string: String,
}

impl TemplateDesc {
    pub fn from(s: String) -> Result<TemplateDesc, String> {
        create_template_from_string(s)
    }
}

pub fn create_template_from_string(s: String) -> Result<TemplateDesc, String> {
    // _-transitivity-preverb-root-thematic_vowel

    let segments = s.split('-').collect::<Vec<&str>>();
    if segments.len() == 5 {
        return Err(format!(
            "The string must have 5 segments, instead it has {}",
            segments.len()
        ));
    }

    if segments[0] != "спр" {
        return Err(format!(
            "The string must start with 'спр', instead it starts with {}",
            segments[0]
        ));
    }

    let transitivity = extract_transitivity(segments[1]).unwrap();
    let preverb = extract_preverb(segments[2]);
    let (fc, v, lc, alternating) = extract_root(segments[3]);
    let thematic_vowel = extract_thematic_vowel(segments[4]).unwrap();

    if fc.is_none() && transitivity == Transitivity::Transitive {
        return Err(
            "The first consonant is not allowed to be None if the verb is transitive.".to_owned(),
        );
    }

    let template_desc = TemplateDesc {
        transitivity,
        preverb,
        stem: VerbStem {
            first_consonant: fc,
            vowel: v,
            last_consonant: lc,
            is_alternating: alternating,
            thematic_vowel,
            string: segments[3].to_owned(),
        },
        original_string: s.clone(),
    };
    Ok(template_desc)
}
