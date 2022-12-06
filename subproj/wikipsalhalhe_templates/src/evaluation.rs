use crate::ortho;
use crate::template;
use crate::PersonMarker;
use crate::{Case, Morpheme, MorphemeKind, Number, Person, Preverb, PreverbSoundForm};
use std::collections::VecDeque;

pub fn morphemes_to_string(morphemes: &VecDeque<Morpheme>) -> String {
    let s = morphemes
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join("-");
    s
}

fn evaluate_person_marker_(
    marker: &PersonMarker,
    morphemes: &VecDeque<Morpheme>,
    i: usize,
) -> String {
    let morpheme = morphemes[i].clone();
    let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

    let mut is_start;
    let mut is_after_vowel;
    let mut is_before_vowel;
    let mut is_after_consonant;
    let mut is_next_consonant_voiced;

    match morpheme_prev_kind {
        Some(..) => {}
        None => {
            is_start = true;
            is_after_vowel = false;
            is_after_consonant = false;
        }
    }

    match morpheme_next_kind {
        Some(mk) if mk.first_letter().unwrap().is_consonant() => {
            is_before_vowel = false;
            is_next_consonant_voiced =
                mk.first_letter().unwrap().get_voiceness() == ortho::Voiceness::Voiced;
        }
        Some(mk) if mk.first_letter().unwrap().is_vowel() => {
            is_before_vowel = true;
        }
        None => unreachable!("PersonMarker must be followed by a morpheme"),
        x => unreachable!("{:?}", x),
    }

    let s = String::new();

    s
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

    let morpheme = morphemes[i].clone();
    let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

    let is_raj_imperative = morphemes.iter().any(|m| {
        if let MorphemeKind::RajImperative = &m.kind {
            return true;
        }
        false
    });

    let is_third_plural = (marker.person, marker.number) == (Person::Third, Number::Plural);
    let is_second_singular = (marker.person, marker.number) == (Person::Second, Number::Singular);

    match marker.case {
        Case::Ergative => {
            // Because ergative markers are only one letter, we do this
            let first_letter = &ortho::parse(&marker.get_base_string())[0];

            if is_third_plural {
                let x = match morpheme_prev_kind {
                    Some(..) => "а",
                    None => "я",
                };
                x.to_owned()
            } else {
                match (morpheme_prev_kind, morpheme_next_kind) {
                    (_, Some(mnk)) if is_raj_imperative => {
                        let mut new_marker = *marker;
                        if mnk == &MorphemeKind::RajImperative {
                            if (marker.person, marker.number) == (Person::Second, Number::Singular)
                            {
                                new_marker.case = Case::Absolutive;
                            }
                        } else {
                            new_marker.case = Case::Absolutive;
                        }
                        new_marker.get_string()
                    }
                    (_, Some(MorphemeKind::Generic(base))) if base == &"о" => {
                        if (marker.person, marker.number) == (Person::Third, Number::Singular) {
                            // 'и' + 'о' -> 'е'
                            "е".to_owned()
                        } else {
                            marker.get_string()
                        }
                    }
                    // If it is the start of the verb with a negative prefix, the ergative markers get a 'ы'
                    // They basically look like normal absolutive markers
                    // Howver if there is something before that, like a preverb, the ergative markers behave normal again,
                    // except the second person singular, which stays 'у' instead of becoming 'б'/'п'.
                    (x, Some(negative @ MorphemeKind::NegationPrefix)) => {
                        assert!(
                            [Case::Ergative, Case::Absolutive].contains(&marker.case),
                            "Case marker cas to be absolutive or ergative, it was however {:?}",
                            marker.case
                        );

                        if let None = x {
                            let mut new_marker = *marker;
                            new_marker.case = Case::Absolutive;

                            new_marker.get_string()
                        } else if let Some(MorphemeKind::Preverb(preverb)) = x {
                            use ortho::{Manner::*, Place::*};
                            match &first_letter.kind {
                                ortho::LetterKind::Consonant(consonant) => {
                                    let mut consonant = consonant.clone();
                                    consonant.voiceness =
                                        negative.first_letter().unwrap().get_voiceness();
                                    let mut empenthetic = "".to_owned();

                                    // if 'п' or 'б', make it 'у'.
                                    if (consonant.place, consonant.manner) == (Labial, Plosive) {
                                        consonant.manner = Approximant;
                                        let lc = preverb.last_consonant().unwrap();
                                        if [Velar, Uvular, Glottal].contains(&lc.place) {
                                            empenthetic = "ы".to_owned();
                                        }
                                    }

                                    empenthetic + &consonant.to_string()
                                }
                                ortho::LetterKind::Combi(..) => first_letter.to_string(),
                                _ => unreachable!(),
                            }
                        } else {
                            panic!("Negative prefix must be preceded by a preverb or nothing")
                            //marker.get_string()
                        }
                    }
                    (_, Some(MorphemeKind::Stem(stem, base))) => match &first_letter.kind {
                        ortho::LetterKind::Consonant(consonant) => {
                            // let x = preverb.get_first_letter().get_voiceness();
                            let mut consonant = consonant.clone();
                            use ortho::Voiceness as OV;
                            use template::FirstConsonant as TV;
                            consonant.voiceness = match &stem.first_consonant.clone().unwrap() {
                                TV::Unvoiced => OV::Voiceless,
                                TV::Voiced => OV::Voiced,
                                _ => unreachable!(),
                            };

                            consonant.to_string()
                        }
                        ortho::LetterKind::Combi(..) => first_letter.to_string(),
                        _ => unreachable!(),
                    },
                    (None, Some(..)) => match &first_letter.kind {
                        ortho::LetterKind::Consonant(consonant) => {
                            let consonant = consonant.clone();
                            consonant.to_string()
                        }
                        ortho::LetterKind::Combi(..) => first_letter.to_string(),
                        _ => unreachable!(),
                    },
                    x => unreachable!("{:?}", x),
                }
            }
        }
        Case::Absolutive => {
            let x = match (morpheme_prev_kind, morpheme_next_kind) {
                (None, Some(generic @ MorphemeKind::Generic(..)))
                    if generic.first_letter().unwrap().is_vowel() =>
                {
                    let x = marker.get_base_string();
                    let xxx = if x.ends_with('ы') {
                        let mut chars = x.chars();
                        chars.next_back();
                        let reduced = chars.as_str().to_string();
                        reduced
                    } else {
                        x
                    };
                    xxx
                }
                _ => marker.get_base_string(),
            };
            //m.get_base_string()
            x
        }
        Case::Oblique => unimplemented!(),
    }
}
fn evaluate_preverb(preverb: &Preverb, morphemes: &VecDeque<Morpheme>, i: usize) -> String {
    let p = preverb.clone();
    let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

    // let mut result = String::new();
    use PreverbSoundForm::*;
    match &morpheme_next.unwrap().kind {
        MorphemeKind::PersonMarker(marker) => {
            assert!(marker.case == Case::Ergative);

            // let base = marker.get_base_string();
            let first_letter = ortho::parse(&marker.get_base_string())[0].clone();
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
                    let x = match morpheme_next_next_kind {
                        Some(MorphemeKind::Generic(base)) if base == &"о" => {
                            preverb.get_form(&Reduced)
                        }
                        _ => preverb.get_form(&Full),
                    };
                    x
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
pub fn evaluate_morphemes(morphemes: &VecDeque<Morpheme>) -> String {
    let mut result = String::new();

    let is_raj_imperative = morphemes.iter().any(|m| {
        if let MorphemeKind::RajImperative = &m.kind {
            return true;
        }
        false
    });

    for (i, morpheme) in morphemes.iter().enumerate() {
        let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
        let morpheme_next = morphemes.get(i + 1);
        let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
        let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

        let _is_first_morpheme = morpheme_prev.is_none();
        let _is_last_morpheme = morpheme_next.is_none();

        let x = match morpheme.kind {
            MorphemeKind::PersonMarker(marker) => evaluate_person_marker(&marker, morphemes, i),
            MorphemeKind::Stem(ref stem, ref base) => {
                // Because of orthography and phonological rules the ы letter, /ə/ sound, has to be treated in a special way.
                //
                let tv = template::treat_thematic_vowel(&stem.thematic_vowel, stem);

                let tv = match tv.as_ref() {
                    "ы" => {
                        let tv = match (&morpheme_prev_kind, &morpheme_next_kind) {
                            (None, None) => tv,
                            (Some(MorphemeKind::PersonMarker(marker)), None)
                                if marker.case == Case::Ergative =>
                            {
                                let morpheme_prev_prev = i.checked_sub(2).map(|i| &morphemes[i]);
                                // morpheme_prev_prev.map(|m| tv).unwrap_or("".to_owned())
                                morpheme_prev_prev.map(|m| "".to_owned()).unwrap_or(tv)
                            }
                            (_, Some(MorphemeKind::Generic(..)))
                                if morpheme_next.unwrap().first_letter().unwrap().is_nasal() =>
                            {
                                tv
                            }
                            _ => "".to_owned(),
                        };
                        tv
                    }
                    v @ ("э" | "") => v.to_owned(),
                    x => unreachable!("{:?}", x),
                };

                morpheme.to_string() + &tv
            }
            MorphemeKind::Preverb(ref preverb) => evaluate_preverb(preverb, morphemes, i),
            MorphemeKind::NegationPrefix => "мы".to_owned(),
            MorphemeKind::Generic(ref base) if base == &"о" => {
                match (morpheme_prev_kind, morpheme_next) {
                    (None, Some(..)) => "мэ".to_owned(),
                    (Some(MorphemeKind::PersonMarker(marker)), Some(..))
                        if marker.case == Case::Ergative && marker.person == Person::Third =>
                    {
                        match marker.number {
                            Number::Singular => "".to_owned(),
                            Number::Plural => "".to_owned(),
                        }
                    }
                    _ => morpheme.to_string(),
                }
            }
            MorphemeKind::Generic(..) | MorphemeKind::RajImperative => morpheme.to_string(), // _ => unimplemented!(),
        };
        result += &x;
    }
    result
}
