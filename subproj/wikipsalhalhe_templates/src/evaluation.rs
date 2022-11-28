use crate::{
    treat_thematic_vowel, Case, ConsonantKind, Morpheme, MorphemeKind, Number, Person, PreverbForm,
    SoundForm,
};
use std::collections::VecDeque;

pub fn evaluate_morphemes(morphemes: &VecDeque<Morpheme>) -> String {
    let mut result = String::new();

    let is_raj_imperative = morphemes.iter().any(|m| {
        if let MorphemeKind::RajImperative = &m.kind {
            return true;
        }
        return false;
    });

    for (i, morpheme) in morphemes.iter().enumerate() {
        let morpheme_prev = if let Some(i) = i.checked_sub(1) {
            morphemes.get(i)
        } else {
            None
        };
        let morpheme_next = morphemes.get(i + 1);
        let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
        let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

        let _is_first_morpheme = morpheme_prev.is_none();
        let _is_last_morpheme = morpheme_next.is_none();

        match morpheme.kind {
            MorphemeKind::PersonMarker(marker) => {
                let is_third_plural =
                    (marker.person, marker.number) == (Person::Third, Number::Plural);
                let is_second_singular =
                    (marker.person, marker.number) == (Person::Second, Number::Singular);

                let mut m = marker;
                let ss = match marker.case {
                    Case::Ergative => {
                        let x = if is_third_plural {
                            // This makes 'я' to 'а' before a consonant
                            match morpheme_prev_kind {
                                Some(..) => SoundForm::AfterConsonant,
                                None => SoundForm::Base,
                            }
                        } else {
                            match (morpheme_prev_kind, morpheme_next_kind) {
                                // If it is the start of the verb with a negative prefix, the ergative markers get a 'ы'
                                // They basically look like normal absolutive markers
                                (None, Some(MorphemeKind::NegationPrefix))
                                | (None, Some(MorphemeKind::RajImperative)) => {
                                    SoundForm::NegativePrefixBase
                                }
                                (None, Some(MorphemeKind::Preverb(..))) if is_raj_imperative => {
                                    SoundForm::NegativePrefixBase
                                }
                                // Howver if there is something before that, like a preverb, the ergative markers behave normally again,
                                // except the second person singular, which stays 'у' instead of becoming 'б'/'п'.
                                (Some(..), Some(MorphemeKind::NegationPrefix)) => {
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
                    Case::Absolutive => {
                        m.form = SoundForm::Base;
                        m.get_base()
                    } // PersonMarkerCase::Oblique => unimplemented!(),
                };
                result += &ss;
            }
            MorphemeKind::Stem(ref stem) => {
                let tv = treat_thematic_vowel(&stem.thematic_vowel, stem);

                let tv = match (&morpheme_prev_kind, &morpheme_next_kind) {
                    (None, None) => tv,
                    (Some(MorphemeKind::PersonMarker(marker)), None)
                        if marker.case == Case::Ergative =>
                    {
                        let morpheme_prev_prev = i.checked_sub(2).map(|i| &morphemes[i]);
                        if morpheme_prev_prev.is_none() {
                            tv
                        } else {
                            "".to_owned()
                        }
                    }
                    _ => "".to_owned(),
                };

                result += &(morpheme.base.clone() + &tv);
            }
            MorphemeKind::Preverb(ref preverb) => {
                let mut p = preverb.clone();

                match &morpheme_next.unwrap().kind {
                    MorphemeKind::PersonMarker(marker) => {
                        match (marker.person, marker.number, marker.case) {
                            (Person::Third, _, Case::Ergative) => {
                                p.form = PreverbForm::BeforeVowel;
                            }
                            (Person::Second, Number::Singular, Case::Ergative) => {
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
                    MorphemeKind::Stem(..) => {
                        p.form = PreverbForm::Full;
                    }
                    MorphemeKind::NegationPrefix => {
                        p.form = PreverbForm::Full;
                    }
                    MorphemeKind::RajImperative => {
                        p.form = PreverbForm::BeforeVowel;
                    }
                    x => unimplemented!("{:?}", x),
                }
                result += &preverb.get_string(p.form);
            }
            MorphemeKind::NegationPrefix => {
                result += "мы";
            }
            MorphemeKind::Generic | MorphemeKind::RajImperative => {
                result += &morpheme.base;
            } // _ => unimplemented!(),
        }
    }
    result
}
