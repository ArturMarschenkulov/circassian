use crate::morpho::{Preverb, Transitivity};

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

#[derive(Debug, Clone, PartialEq)]
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
impl std::fmt::Display for ThematicVowel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ThematicVowel::A => write!(f, "э"),
            ThematicVowel::Y => write!(f, "ы"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FirstConsonant {
    Unvoiced, // 'д'
    Voiced,   // 'жь'
    Wy,       // 'у'
              // Yy, // 'й'
}
#[derive(Debug, Clone, PartialEq)]
pub enum LastConsonant {
    Ordinary, // 'д'
    Velar,    // 'т'
    Labial,   // 'л'
    Yy,       // 'й'
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerbStem {
    pub first_consonant: Option<FirstConsonant>,
    pub vowel: VowelKind,
    pub last_consonant: LastConsonant,
    pub is_alternating: bool,
    pub thematic_vowel: ThematicVowel,
    pub string: String,
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

fn get_transitivity_str(s: &str) -> &str {
    let segments = s.split('-').collect::<Vec<&str>>();
    segments[1]
}

fn extract_transitivity(s: &str) -> Option<Transitivity> {
    let segments = s.split('-').collect::<Vec<&str>>();
    match s {
        "лъэмыӏ" => Some(Transitivity::Intransitive),
        "лъэӏ" => Some(Transitivity::Transitive),
        _ => None,
    }
}
fn get_preverb_str(s: &str) -> &str {
    let segments = s.split('-').collect::<Vec<&str>>();
    segments[2]
}
fn extract_preverb(s: &str) -> Option<Preverb> {
    match s {
        "0" => None,
        _ => Some(Preverb::new(&s.to_owned())),
    }
}

pub fn get_root_str(s: &str) -> &str {
    let segments = s.split('-').collect::<Vec<&str>>();
    segments[3]
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
    let c = &root.chars().nth(0).unwrap();
    let (root, fc) = if !['б', '0'].contains(c) {
        // this means it's transitive.
        let c = &root.chars().nth(0).unwrap();
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

    let c = &root.chars().nth(0).unwrap();
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

    let c = &root.chars().nth(0).unwrap();
    let (root, lc) = if !['б', '0'].contains(c) {
        // this means it's transitive.
        let c = &root.chars().nth(0).unwrap();
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

    let alternating = if root.ends_with("эа") { true } else { false };
    (fc, v, lc, alternating)
}

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
    let transitivity_str = get_transitivity_str(&s);
    let preverb_str = get_preverb_str(&s);
    let root_str = get_root_str(&s);

    let transitivity = extract_transitivity(&transitivity_str).unwrap_or_else(|| {
        println!("The transitivity is not either 'лъэмыӏ' or 'лъэӏ'");
        panic!("");
    });
    let preverb = extract_preverb(&preverb_str);

    let thematic_vowel = match segments.last() {
        Some(&"э") => ThematicVowel::A,
        Some(&"ы") => ThematicVowel::Y,
        _ => {
            println!("The last string isn't either 'э' or 'ы'");
            return None;
        }
    };

    let (fc, v, lc, alternating) = extract_root(&root_str);

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
    Some(template_desc)
}
