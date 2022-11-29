use crate::{Preverb, Transitivity, VowelKind};

/// It's basically there to treat stems ending on у which labializes the consonants before it.
/// This also indicates an implicit ы.
pub fn treat_thematic_vowel(tv: &ThematicVowel, vs: &VerbStem) -> String {
    match (&tv, &vs.last_consonant) {
        (ThematicVowel::Y, ConsonantKind::Labialized) => "",
        (ThematicVowel::Y, _) => "ы",
        _ => "э",
    }
    .to_owned()
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsonantKind {
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
pub enum ThematicVowel {
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

/// Here is the information stored about the verb stem.
/// It is extracted from the template string.
/// In the Kabardian language itself, all stems are mostly treated the same, however because of the orthographical system
/// there are some difference how those stems are treated.
#[derive(Debug, Clone, PartialEq)]
pub struct VerbStem {
    pub first_consonant: ConsonantKind,
    vowel: VowelKind,
    last_consonant: ConsonantKind,
    pub thematic_vowel: ThematicVowel,
    string: String,
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

#[derive(Debug, Clone)]
pub struct TemplateDesc {
    pub transitivity: Transitivity,
    pub preverb: Option<Preverb>,
    pub stem: VerbStem,
    pub original_string: String,
}

pub fn create_template_from_string(s: String) -> Option<TemplateDesc> {
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
        _ => Some(Preverb::new(&segments[2].to_owned())),
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
