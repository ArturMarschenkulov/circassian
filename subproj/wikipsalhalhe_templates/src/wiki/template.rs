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

impl From<&ortho::Vowel> for ThematicVowel {
    fn from(vowel: &ortho::Vowel) -> Self {
        match vowel {
            ortho::Vowel::A => ThematicVowel::A,
            ortho::Vowel::Y => ThematicVowel::Y,
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
    pub fn to_voiceness(&self) -> ortho::Voiceness {
        match self {
            FirstConsonant::Unvoiced => ortho::Voiceness::Voiceless,
            FirstConsonant::Voiced => ortho::Voiceness::Voiced,
            FirstConsonant::Wy => ortho::Voiceness::Voiced,
        }
    }
}

impl From<&ortho::Consonant> for FirstConsonant {
    fn from(consonant: &ortho::Consonant) -> Self {
        use ortho::*;
        use Manner::*;
        use Place::*;
        match (consonant.place, consonant.manner, consonant.voiceness) {
            (Labial, Approximant, _) => FirstConsonant::Wy,
            (_, _, Voiceness::Voiced) => FirstConsonant::Voiced,
            _ => FirstConsonant::Unvoiced,
            // _ => panic!("The consonant {:?} is not a first consonant.", consonant),
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

impl From<&ortho::Consonant> for LastConsonant {
    fn from(consonant: &ortho::Consonant) -> Self {
        use ortho::*;
        use Manner::*;
        use Place::*;
        match (consonant.place, consonant.manner, consonant.is_labialized) {
            (_, _, true) => LastConsonant::Labial,
            (Velar, _, _) => LastConsonant::Velar,
            (Palatal, Approximant, _) => LastConsonant::Yy,
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
    pub fn new(s: &str, transitivity: morpho::Transitivity) -> Self {
        use ortho::Letter as L;
        let letters = ortho::parse(s).unwrap();
        assert!(!letters.is_empty(), "The verb stem can't be empty.");

        let vowel = match letters.len() {
            1 => VowelKind::Without,
            _ => VowelKind::With,
        };

        let last_letter = letters.last().unwrap();
        let first_letter = letters.first().unwrap();

        let thematic_vowel = match &last_letter {
            L::Vowel(vowel) => ThematicVowel::from(vowel),
            _ => ThematicVowel::Y,
        };

        let last_consonant = letters
            .iter()
            .rev()
            .find(|l| l.is_consonant())
            .map(|l| {
                if let L::Consonant(consonant) = &l {
                    LastConsonant::from(consonant)
                } else {
                    panic!("The letter {:?} is not a consonant.", l);
                }
            })
            .unwrap();

        // The first consonant is only relevant for transitive verbs.
        let first_consonant = match transitivity {
            Transitivity::Intransitive => None,
            _ => match &first_letter {
                L::Consonant(consonant) => Some(FirstConsonant::from(consonant)),
                _ => panic!("The letter {:?} is not a consonant.", first_letter),
            },
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
        _ => Err(format!(
            "The thematic vowel must be either 'э' or 'ы', instead it is {}",
            s
        )),
    }
}

fn extract_preverb(s: &str) -> Option<Preverb> {
    match s {
        "0" => None,
        _ => Some(Preverb::try_from(s.to_owned().as_str()).unwrap()),
    }
}
fn split_and_combine(input: &str, combined_chars: &Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let mut matched = false;
        for combined in combined_chars {
            if input[i..].starts_with(combined) {
                result.push(combined.to_string());
                i += combined.len();
                matched = true;
                break;
            }
        }
        if !matched {
            let c = input[i..].chars().next().unwrap(); // Get the next char
            result.push(c.to_string());
            i += c.len_utf8();
        }
    }

    result
}

fn extract_root_form(
    root_form: &str,
) -> Result<(Option<FirstConsonant>, VowelKind, LastConsonant, bool), String> {
    fn parse_vowel_kind(s: &str) -> Result<VowelKind, String> {
        match s {
            "б" => Ok(VowelKind::With),
            "0" => Ok(VowelKind::Without),
            _ => Err(format!(
                "The vowel kind must be either 'б' or '0', but it is {}.",
                s
            )),
        }
    }
    fn parse_first_consonant_kind(s: &str) -> Result<FirstConsonant, String> {
        match s {
            "д" => Ok(FirstConsonant::Unvoiced),
            "жь" => Ok(FirstConsonant::Voiced),
            "у" => Ok(FirstConsonant::Wy),
            _ => Err(format!(
                "The first consonant kind must be either 'д', 'жь' or 'у', but it is {}.",
                s
            )),
        }
    }
    fn parse_last_consonant_kind(s: &str) -> Result<LastConsonant, String> {
        match s {
            "д" => Ok(LastConsonant::Ordinary),
            "т" => Ok(LastConsonant::Velar),
            "л" => Ok(LastConsonant::Labial),
            _ => Err(format!(
                "The last consonant kind must be either 'д', 'т' or 'л', but it is {}.",
                s
            )),
        }
    }
    let letters: Vec<String> = split_and_combine(root_form, &vec!["жь", "эа"]);
    println!("LETTERS: {:?}", letters);

    let accepted_lengths = vec![2, 3, 4];
    if !accepted_lengths.contains(&letters.len()) {
        return Err(format!(
            "The root form must be 2, 3 or 4 characters long. Got {} of {}",
            letters.len(),
            root_form
        ));
    }

    let is_alternating = letters.last().unwrap().ends_with("эа");

    if letters.len() == 2 && !is_alternating || letters.len() == 3 && is_alternating {
        let vowel = parse_vowel_kind(&letters[0]).unwrap();
        let last_consonant = parse_last_consonant_kind(&letters[1]).unwrap();
        return Ok((None, vowel, last_consonant, is_alternating));
    } else if letters.len() == 3 && !is_alternating || letters.len() == 4 && is_alternating {
        let first_consonant = parse_first_consonant_kind(&letters[0]).unwrap();
        let vowel = parse_vowel_kind(&letters[1]).unwrap();
        let last_consonant = parse_last_consonant_kind(&letters[2]).unwrap();
        return Ok((Some(first_consonant), vowel, last_consonant, is_alternating));
    } else {
        unimplemented!("It is not implemented yet. Work in progress.");
    }
}

#[derive(Debug, Clone)]
pub struct TemplateDesc {
    pub transitivity: Transitivity,
    pub preverb: Option<Preverb>,
    pub stem: VerbStem,
    pub original_string: String,
}

impl TryFrom<TemplateDesc> for String {
    type Error = String;
    fn try_from(template_desc: TemplateDesc) -> Result<String, String> {
        if template_desc.stem.first_consonant.is_some()
            && template_desc.transitivity == Transitivity::Intransitive
        {
            return Err(format!(
                "Intransitive verbs cannot have first consonant. Got {:?}",
                template_desc
            ));
        }

        let mut s = String::new();
        s.push_str("спр-");
        s.push_str(match template_desc.transitivity {
            Transitivity::Intransitive => "лъэмыӏ",
            Transitivity::Transitive => "лъэӏ",
        });

        s.push('-');

        match template_desc.preverb {
            Some(preverb) => s.push_str(&preverb.base),
            None => s.push('0'),
        };

        s.push('-');

        if let Some(fc) = template_desc.stem.first_consonant {
            match fc {
                FirstConsonant::Unvoiced => s.push('д'),
                FirstConsonant::Voiced => s.push_str("жь"),
                FirstConsonant::Wy => s.push('у'),
            }
        }

        s.push(match template_desc.stem.vowel {
            VowelKind::With | VowelKind::Alternating => 'б',
            VowelKind::Without => '0',
        });
        s.push(match template_desc.stem.last_consonant {
            LastConsonant::Ordinary => 'д',
            LastConsonant::Velar => 'т',
            LastConsonant::Labial => 'л',
            LastConsonant::Yy => 'й',
        });
        if let VowelKind::Alternating = template_desc.stem.vowel {
            s.push_str("эа");
        }

        s.push('-');

        s.push(match template_desc.stem.thematic_vowel {
            ThematicVowel::A => 'э',
            ThematicVowel::Y => 'ы',
        });

        Ok(s)
    }
}
impl TryFrom<&str> for TemplateDesc {
    type Error = String;
    fn try_from(s: &str) -> Result<TemplateDesc, String> {
        let segments = s.split('-').collect::<Vec<&str>>();
        if segments.is_empty() {
            return Err("The string is empty.".to_owned());
        }

        if segments[0] != "спр" {
            return Err(format!(
                "The string must start with 'спр', instead it starts with {}",
                segments[0]
            ));
        }

        if segments.len() != 5 {
            return Err(format!(
                "The string must have 5 segments, instead it has {}",
                segments.len()
            ));
        }

        let transitivity = extract_transitivity(segments[1]).unwrap();
        let preverb = extract_preverb(segments[2]);
        let (fc, v, lc, alternating) = extract_root_form(segments[3]).unwrap();
        let thematic_vowel = extract_thematic_vowel(segments[4]).unwrap();

        if fc.is_none() && transitivity == Transitivity::Transitive {
            return Err(
                "The first consonant is not allowed to be None if the verb is transitive."
                    .to_owned(),
            );
        }
        if fc.is_some() && transitivity == Transitivity::Intransitive {
            return Err(
                "The first consonant is not allowed to be Some if the verb is intransitive."
                    .to_owned(),
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
            original_string: s.to_owned(),
        };
        Ok(template_desc)
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_split_and_combine() {
        let combined_chars = vec!["жь", "эа"];

        let input = "жь0й";
        let result = split_and_combine(input, &combined_chars);
        assert_eq!(result, vec!["жь", "0", "й"]);

        let input = "дблэа";
        let result = split_and_combine(input, &combined_chars);
        assert_eq!(result, vec!["д", "б", "л", "эа"]);
    }

    fn test_templates_creation() {
        let mut map = std::collections::HashMap::<String, TemplateDesc>::new();

        map.insert(
            "спр-лъэмыӏ-0-0д-э".to_owned(),
            TemplateDesc {
                transitivity: Transitivity::Intransitive,
                preverb: None,
                stem: VerbStem {
                    first_consonant: None,
                    vowel: VowelKind::Without,
                    last_consonant: LastConsonant::Ordinary,
                    is_alternating: false,
                    thematic_vowel: ThematicVowel::A,
                    string: "0д".to_owned(),
                },
                original_string: "спр-лъэмыӏ-0-0д-э".to_owned(),
            },
        );
        map.insert(
            "спр-лъэмыӏ-0-0д-ы".to_owned(),
            TemplateDesc {
                transitivity: Transitivity::Intransitive,
                preverb: None,
                stem: VerbStem {
                    first_consonant: None,
                    vowel: VowelKind::Without,
                    last_consonant: LastConsonant::Ordinary,
                    is_alternating: false,
                    thematic_vowel: ThematicVowel::Y,
                    string: "0д".to_owned(),
                },
                original_string: "спр-лъэмыӏ-0-0д-э".to_owned(),
            },
        );
        map.insert(
            "спр-лъэмыӏ-0-0л-ы".to_owned(),
            TemplateDesc {
                transitivity: Transitivity::Intransitive,
                preverb: None,
                stem: VerbStem {
                    first_consonant: None,
                    vowel: VowelKind::Without,
                    last_consonant: LastConsonant::Labial,
                    is_alternating: false,
                    thematic_vowel: ThematicVowel::Y,
                    string: "0л".to_owned(),
                },
                original_string: "спр-лъэмыӏ-0-0л-ы".to_owned(),
            },
        );
        
        // let _possible_templates = [
        //     "спр-лъэмыӏ-0-0д-э",
        //     "спр-лъэмыӏ-0-0д-ы",
        //     "спр-лъэмыӏ-0-0л-ы",
        //     "спр-лъэмыӏ-0-0т-ы",
        //     "спр-лъэмыӏ-0-бд-э",
        //     "спр-лъэмыӏ-0-бдэа-э",
        //     "спр-лъэмыӏ-0-бт-ы",
        //     "спр-лъэмыӏ-0-бй-ы",
        //     "спр-лъэӏ-0-дблэа-ы",
        //     "спр-лъэӏ-0-дбд-ы",
        //     "спр-лъэӏ-0-жь0й-ы",
        //     "спр-лъэӏ-0-д0д-э",
        //     "спр-лъэӏ-0-убт-ы",
        //     "спр-лъэӏ-0-д0д-ы",
        //     "спр-лъэӏ-0-д0л-ы",
        //     // "спр-лъэмыӏ-е-бд-ы",
        // ];
        for (k, must_be_template) in map.iter() {
            let gen_template = TemplateDesc::try_from(k.as_str()).unwrap();
            let mut is_equal = true;
            if gen_template.transitivity != must_be_template.transitivity {
                is_equal = false;
            }
            if gen_template.preverb != must_be_template.preverb {
                is_equal = false;
            }
            if gen_template.stem.first_consonant != must_be_template.stem.first_consonant {
                is_equal = false;
            }
            if gen_template.stem.vowel != must_be_template.stem.vowel {
                is_equal = false;
            }
            if gen_template.stem.last_consonant != must_be_template.stem.last_consonant {
                is_equal = false;
            }
            if gen_template.stem.thematic_vowel != must_be_template.stem.thematic_vowel {
                is_equal = false;
            }
            if gen_template.original_string != must_be_template.original_string {
                is_equal = false;
            }
            assert!(is_equal);

            let string = String::try_from(gen_template).unwrap();
            assert_eq!(string, k.to_owned());
        }
    }
}
