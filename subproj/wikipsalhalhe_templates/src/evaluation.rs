use crate::morpho::{
    Case, Morpheme, MorphemeKind, Number, Person, PersonMarker, Preverb, PreverbSoundForm,
};
use crate::ortho::{self, Consonant};
use crate::wiki::template::ThematicVowel;
use std::collections::VecDeque;

pub fn morphemes_to_string(morphemes: &VecDeque<Morpheme>) -> String {
    let s = morphemes
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join("-");
    s
}

// fn evaluate_person_marker_(
//     marker: &PersonMarker,
//     morphemes: &VecDeque<Morpheme>,
//     i: usize,
// ) -> String {
//     let morpheme = morphemes[i].clone();
//     let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
//     let morpheme_next = morphemes.get(i + 1);
//     let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
//     let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

//     let is_start;
//     let is_after_vowel;
//     let is_before_vowel;
//     let is_after_consonant;
//     let is_next_consonant_voiced;

//     match morpheme_prev_kind {
//         Some(..) => {}
//         None => {
//             is_start = true;
//             is_after_vowel = false;
//             is_after_consonant = false;
//         }
//     }

//     match morpheme_next_kind {
//         Some(mk) if mk.first_letter().unwrap().is_consonant() => {
//             is_before_vowel = false;
//             is_next_consonant_voiced =
//                 mk.first_letter().unwrap().get_voiceness() == ortho::Voiceness::Voiced;
//         }
//         Some(mk) if mk.first_letter().unwrap().is_vowel() => {
//             is_before_vowel = true;
//         }
//         None => unreachable!("PersonMarker must be followed by a morpheme"),
//         x => unreachable!("{:?}", x),
//     }

//     String::new()
// }

fn ja_form(is_after_consonant: bool) -> String {
    if is_after_consonant {
        "а".to_owned()
    } else {
        "я".to_owned()
    }
}
fn give_epenthetic_if_needed(c_0: &Consonant, c_1: &Consonant) -> String {
    let is_wy = (ortho::Manner::Approximant, ortho::Place::Labial) == (c_0.manner, c_0.place);
    let needs_it = c_1.needs_epenthetic_y();
    if is_wy && needs_it {
        "ы".to_owned()
    } else {
        "".to_owned()
    }
}
fn evaluate_person_marker(
    marker: &PersonMarker,
    morphemes: &VecDeque<Morpheme>,
    i: usize,
) -> String {
    /*
    NOTE:
        I think it would be useful to treat 3-person markers in a special way.
        The 2-person singular marker is also special.
    */

    let _morpheme = morphemes[i].clone();
    let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

    let has_raj_prefix = morphemes.iter().any(|m| match &m.kind {
        MorphemeKind::RajImperative => true,
        _ => false,
    });
    // let has_o_prefix = morphemes.iter().any(|m| match &m.kind {
    //     MorphemeKind::Generic(base) => base == "о",
    //     _ => false,
    // });

    // 'я' is different in that it is only every phonologically changed if there is something before it. That form is 'а'.
    // Any vowel after it gets swallowed. That is я + о = я,

    match (morpheme_prev_kind, morpheme_next_kind) {
        (x, Some(morph_next)) if has_raj_prefix => {
            // Raj imperatives can have a person marker only at the very beginning.
            //
            assert_eq!(marker.case, Case::Ergative);
            assert!(x.is_none());

            let mut new_marker = *marker;
            if morph_next == &MorphemeKind::RajImperative && marker.is_second_singular() {
                new_marker.case = Case::Absolutive;
            } else if morph_next == &MorphemeKind::RajImperative && !marker.is_second_singular() {
                new_marker.case = Case::Ergative;
            } else {
                new_marker.case = Case::Absolutive;
            }
            new_marker.base_string()
        }
        (_, None) if has_raj_prefix => {
            unreachable!("Logic error: Raj imperative must be followed by a morpheme")
        }
        (mp, Some(MorphemeKind::Generic(base))) if base == "о" => {
            if marker.is_third() {
                if marker.is_third_singular_ergative() {
                    // 'и' + 'о' -> 'е'
                    "е".to_owned()
                } else if marker.is_third_plural_ergative() {
                    ja_form(morpheme_prev_kind.is_some())
                } else {
                    unreachable!()
                }
            } else {
                if marker.is_absolutive() {
                    marker.base_string_as_before_o()
                } else if marker.is_ergative() && mp.is_some() {
                    marker.base_string_as_voiced()
                } else {
                    marker.base_string()
                }
            }
        }
        // If it is the start of the verb with a negative prefix, the ergative markers get a 'ы'
        // They basically look like normal absolutive markers
        // However if there is something before that, like a preverb, the ergative markers behave normal again,
        // except the second person singular, which stays 'у' instead of becoming 'б'/'п'.
        (morpheme_prev, Some(neg_prefix @ MorphemeKind::NegationPrefix)) => {
            assert!(
                [Case::Ergative, Case::Absolutive].contains(&marker.case),
                "Case marker cas to be absolutive or ergative, it was however {:?}",
                marker.case
            );

            assert_ne!((morpheme_prev.is_some(), marker.case), (true, Case::Absolutive), "If the person marker is absolutive and is followed by a negation prefix, then it must be the first morpheme of the verb, it was however {:?}", (morpheme_prev, marker.case));

            if morpheme_prev.is_none() {
                let mut new_marker = *marker;
                new_marker.case = Case::Absolutive;

                new_marker.base_string()
            } else if let Some(MorphemeKind::Preverb(preverb)) = morpheme_prev {
                use ortho::Manner::*;
                match &marker.to_letters()[0].kind {
                    ortho::LetterKind::Consonant(consonant) => {
                        let mut consonant = consonant.clone();
                        consonant.voiceness = neg_prefix.first_letter().unwrap().get_voiceness();
                    
                        // if 'п' or 'б', make it 'у'.
                        if consonant.is_labial_plosive() {
                            consonant.manner = Approximant;
                        };
                        let empenthetic = give_epenthetic_if_needed(
                            &consonant,
                            &preverb.last_consonant().unwrap(),
                        );

                        empenthetic + &consonant.to_string()
                    }
                    ortho::LetterKind::Combi(..) => {
                        if marker.is_third_plural_ergative() {
                            ja_form(morpheme_prev_kind.is_some())
                        } else {
                            marker.to_letters()[0].to_string()
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                panic!("Negative prefix must be preceded by a preverb or nothing")
                //marker.get_string()
            }
        }
        (_, Some(MorphemeKind::Stem(stem, _))) => {
            if marker.is_ergative() {
                match &marker.to_letters()[0].kind {
                    ortho::LetterKind::Consonant(consonant) => {
                        assert_ne!((marker.person, marker.number), (Person::Third, Number::Plural), "Second person singular ergative markers are not allowed to be followed by a stem");
                        // let x = preverb.get_first_letter().get_voiceness();
                        let mut consonant = consonant.clone();
                        consonant.voiceness = stem
                            .first_consonant
                            .as_ref()
                            .unwrap()
                            .to_voiceness()
                            .to_owned();

                        consonant.to_string()
                    }
                    ortho::LetterKind::Combi(..) => {
                        if marker.is_third_plural_ergative() {
                            ja_form(morpheme_prev_kind.is_some())
                        } else {
                            marker.to_letters()[0].to_string()
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                marker.base_string()
            }
        }
        (None, Some(..)) => match &marker.to_letters()[0].kind {
            ortho::LetterKind::Consonant(consonant) => consonant.to_string(),
            ortho::LetterKind::Combi(..) => marker.to_letters()[0].to_string(),
            _ => unreachable!(),
        },
        x => unreachable!("{:?}", x),
    }
}
fn evaluate_preverb(preverb: &Preverb, morphemes: &VecDeque<Morpheme>, i: usize) -> String {
    let _morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    // let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    // let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

    // let mut result = String::new();
    use PreverbSoundForm::*;
    match &morpheme_next.unwrap().kind {
        MorphemeKind::PersonMarker(marker) => {
            assert!(marker.case == Case::Ergative);

            // let base = marker.get_base_string();
            let first_letter = marker.to_letters()[0].clone();
            let x = match first_letter.kind {
                ortho::LetterKind::Combi(..) => preverb.get_form(&BeforeVowel),
                ortho::LetterKind::Consonant(cons)
                    if cons.base == "б"
                        && morphemes
                            .get(i + 2)
                            .map(|m| m.kind == MorphemeKind::NegationPrefix)
                            .unwrap_or(false) =>
                {
                    preverb.get_form(&BeforeVowel)
                }
                _ => {
                    let morpheme_next_next = morphemes.get(i + 2);
                    let morpheme_next_next_kind = morpheme_next_next.map(|x| &x.kind);
                    match morpheme_next_next_kind {
                        Some(MorphemeKind::Generic(base)) if base == "о" => {
                            preverb.get_form(&Reduced)
                        }
                        _ => preverb.get_form(&Full),
                    }
                }
            };
            x
        }
        MorphemeKind::Stem(..) => preverb.get_form(&Full),
        MorphemeKind::NegationPrefix => preverb.get_form(&Full),
        MorphemeKind::RajImperative => preverb.get_form(&BeforeVowel),
        x => unimplemented!("{:?}", x),
    }
}

fn evaluate_stem(
    stem: &crate::wiki::template::VerbStem,
    morphemes: &VecDeque<Morpheme>,
    i: usize,
) -> String {
    let morpheme = &morphemes[i];
    let _morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let _morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = i.checked_sub(1).map(|i| &morphemes[i]).map(|x| &x.kind);
    let morpheme_next_kind = morphemes.get(i + 1).map(|x| &x.kind);
    // Because of orthography and phonological rules the ы letter, /ə/ sound, has to be treated in a special way.
    //
    let thematic_vowel = crate::wiki::template::treat_thematic_vowel(&stem.thematic_vowel, stem);

    let tv = match thematic_vowel.as_ref() {
        "ы" => match (&morpheme_prev_kind, &morpheme_next_kind) {
            (None, None) => thematic_vowel,
            (Some(MorphemeKind::PersonMarker(marker)), None) if marker.case == Case::Ergative => {
                // let morpheme_prev_prev = i.checked_sub(2).map(|i| &morphemes[i]);
                i.checked_sub(2)
                    .map(|i| &morphemes[i])
                    .map(|_| "".to_owned())
                    .unwrap_or(thematic_vowel)
            }
            (_, Some(MorphemeKind::Generic(..)))
                if morpheme_prev_kind
                    .unwrap()
                    .first_letter()
                    .unwrap()
                    .is_nasal() =>
            {
                thematic_vowel
            }
            _ => "".to_owned(),
        },
        v @ "э" => match morpheme_next_kind {
            Some(MorphemeKind::Generic(gen)) => {
                let first_letter_of_next_morpheme = gen.chars().next().unwrap().to_string();
                if first_letter_of_next_morpheme == "а" {
                    "".to_owned()
                } else {
                    v.to_owned()
                }
            }
            _ => v.to_owned(),
        },
        v @ "" => v.to_owned(),
        x => unreachable!("{:?}", x),
    };

    morpheme.to_string() + &tv
}

fn evaluate_o(morphemes: &VecDeque<Morpheme>, i: usize) -> String {
    let morpheme = &morphemes[i];
    let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

    // TODO: Assert whether the next morpheme is a stem

    match (morpheme_prev_kind, morpheme_next_kind) {
        (None, Some(MorphemeKind::Stem(..))) => "мэ".to_owned(),

        // The third person ergative markers и and я, become е and я (no change) respectively, while о goes away.
        (Some(MorphemeKind::PersonMarker(marker)), Some(MorphemeKind::Stem(..)))
            if marker.is_third_ergative() =>
        {
            "".to_owned()
        }

        (_, Some(MorphemeKind::Stem(..))) => morpheme.to_string(),
        _ => unreachable!(),
    }
}
pub fn evaluate_morphemes(morphemes: &VecDeque<Morpheme>) -> String {
    let mut result = String::new();

    let _has_raj_prefix = morphemes.iter().any(|m| match &m.kind {
        MorphemeKind::RajImperative => true,
        _ => false,
    });

    for (i, morpheme) in morphemes.iter().enumerate() {
        let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
        let morpheme_next = morphemes.get(i + 1);
        let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
        let _morpheme_next_kind = morpheme_next.map(|x| &x.kind);

        let _is_first_morpheme = morpheme_prev.is_none();
        let _is_last_morpheme = morpheme_next.is_none();

        let x = match morpheme.kind {
            MorphemeKind::PersonMarker(marker) => evaluate_person_marker(&marker, morphemes, i),
            MorphemeKind::Stem(ref stem, _) => evaluate_stem(stem, morphemes, i),
            MorphemeKind::Preverb(ref preverb) => evaluate_preverb(preverb, morphemes, i),
            MorphemeKind::NegationPrefix => "мы".to_owned(),
            MorphemeKind::Generic(ref base) if base == "о" => evaluate_o(morphemes, i),
            MorphemeKind::Generic(ref base) if base.starts_with('р') && !base.starts_with("рэ") => match morpheme_prev_kind {
                Some(MorphemeKind::Stem(s, _)) if s.thematic_vowel == ThematicVowel::Y => {
                    "ы".to_owned() + &morpheme.to_string()
                }
                _ => morpheme.to_string(),
            },
            MorphemeKind::Generic(ref base) if base.starts_with('н') || base.starts_with("гъ") => match morpheme_prev_kind {
                Some(MorphemeKind::Stem(s, _)) if s.thematic_vowel == ThematicVowel::Y => {
                    "ы".to_owned() + &morpheme.to_string()
                }
                _ => morpheme.to_string(),
            },
            MorphemeKind::Generic(..) | MorphemeKind::RajImperative => morpheme.to_string(), // _ => unimplemented!(),
        };
        result += &x;
    }
    result
}
