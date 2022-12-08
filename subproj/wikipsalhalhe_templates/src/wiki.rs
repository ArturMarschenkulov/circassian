/// This module will contain everything specific to wikipsalhalhe. In the future this may also include the normal wiki.
///
///
mod table;
pub mod template;

use std::collections::VecDeque;

use crate::{
    evaluation,
    morpho::{self, Case, Morpheme, MorphemeKind, Number, Person, PersonMarker, Transitivity},
};

use self::table::Wikitable;

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
fn table_masdar(desc: &template::TemplateDesc) -> String {
    // let root = "{{{псалъэпкъ}}}".to_owned();
    let table_name = "Инфинитив (масдар)".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    table.add("".to_owned());

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ:", polarity));

        let morphemes = morpho::new_masdar(polarity, &desc.preverb, &desc.stem);
        let string = evaluation::evaluate_morphemes(&morphemes);
        table.add(string);
    }
    table.to_string()
}

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
fn table_masdar_personal(desc: &template::TemplateDesc) -> String {
    let table_name = "Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    let subject_case = &desc.transitivity.subject_case();
    let pronouns = match &desc.transitivity {
        Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
        Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
    };

    for pronoun in pronouns.iter() {
        table.add(pronoun.to_string());
    }
    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));

        for number in &[Number::Singular, Number::Plural] {
            for person in &[Person::First, Person::Second, Person::Third] {
                let abs_marker = if subject_case == &Case::Absolutive {
                    PersonMarker::new(*person, *number, Case::Absolutive)
                } else {
                    PersonMarker::new(Person::Third, *number, Case::Absolutive)
                };
                let erg_marker = if subject_case == &Case::Ergative {
                    Some(PersonMarker::new(*person, *number, Case::Ergative))
                } else {
                    None
                };
                let morphemes = morpho::new_masdar_personal(
                    polarity,
                    &desc.preverb,
                    &desc.stem,
                    &abs_marker,
                    &erg_marker,
                );

                let string = evaluation::evaluate_morphemes(&morphemes);
                // println!("{:?}", evaluation::morphemes_to_string(&morphemes));
                table.add(string);
            }
        }
    }
    table.to_string()
}

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
fn table_imperative(desc: &template::TemplateDesc) -> String {
    let table_name = "унафэ наклоненэ".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    for pronoun in ["уэ", "фэ"].iter() {
        table.add(pronoun.to_string());
    }

    let subject_case = &desc.transitivity.subject_case();

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            let abs_marker = if subject_case == &Case::Absolutive {
                PersonMarker::new(Person::Second, *number, Case::Absolutive)
            } else {
                PersonMarker::new(Person::Third, *number, Case::Absolutive)
            };
            let erg_marker = if subject_case == &Case::Ergative {
                Some(PersonMarker::new(Person::Second, *number, Case::Ergative))
            } else {
                None
            };

            let morphemes = morpho::new_imperative(
                polarity,
                &desc.preverb,
                &desc.stem,
                &abs_marker,
                &erg_marker,
            );

            let string = evaluation::evaluate_morphemes(&morphemes);
            table.add(string);
        }
    }
    table.to_string()
}

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
fn table_imperative_raj(desc: &template::TemplateDesc) -> String {
    let mut table = Wikitable::new();
    table.add("Ре-кӀэ унафэ наклоненэ".to_owned());
    let pronouns = match &desc.transitivity {
        Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
        Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
    };
    for pronoun in pronouns.iter() {
        table.add(pronoun.to_string());
    }

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            for person in &[Person::First, Person::Second, Person::Third] {
                let morphemes =
                    morpho::new_imperative_raj(polarity, &desc.preverb, &desc.stem, person, number);
                let string = evaluation::evaluate_morphemes(&morphemes);
                table.add(string);
            }
        }
    }
    table.to_string()
}

fn table_indicative(desc: &template::TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();

    let mut table = Wikitable::new();
    table.add("ЗэраӀуатэ наклоненэ".to_owned());
    let subject_case = &desc.transitivity.subject_case();
    let pronouns = match &desc.transitivity {
        Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
        Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
    };
    for pronoun in pronouns.iter() {
        table.add(pronoun.to_string());
    }

    table.add_row();
    table.add("ит зэман – щыӀэныгъэ".to_owned());
    for number in &[Number::Singular, Number::Plural] {
        for person in &[Person::First, Person::Second, Person::Third] {
            let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
            morphemes.push_back(Morpheme {
                kind: MorphemeKind::Stem(desc.stem.clone(), root.clone()),
                // base: root.clone(),
            });
            morphemes.push_back(Morpheme::new_generic("р"));

            morphemes.push_front(Morpheme::new_generic("о"));

            // Add absolutive person marker
            if subject_case == &Case::Ergative {
                let marker = PersonMarker::new(*person, *number, Case::Ergative);
                let m = Morpheme::new_person_marker(&marker);
                morphemes.push_front(m);
            }
            // Add preverb
            if let Some(preverb) = desc.preverb.clone() {
                let m = Morpheme::new_preverb(&preverb);
                morphemes.push_front(m);
            }
            if subject_case == &Case::Absolutive {
                if (person) != (&Person::Third) {
                    let marker = PersonMarker::new(*person, *number, Case::Absolutive);
                    let m = Morpheme::new_person_marker(&marker);
                    morphemes.push_front(m);
                }
            }
            let string = evaluation::evaluate_morphemes(&morphemes);
            table.add(string);
        }
    }
    table.to_string()
}

fn create_template(desc: template::TemplateDesc) -> String {
    let mut result = "".to_string();
    result += &format!("<!-- Template:Wt/kbd/{} -->\n", desc.original_string);

    // Инфинитив (масдар)
    result += &table_masdar(&desc);
    result += "\n-\n";

    // Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа
    result += &table_masdar_personal(&desc);
    result += "\n-\n";
    // унафэ наклоненэ
    result += &table_imperative(&desc);
    result += "\n-\n";

    // Ре-кӀэ унафэ наклоненэ
    result += &table_imperative_raj(&desc);
    result += "\n-\n";

    result += &table_indicative(&desc);
    result += "\n-\n";

    result += "|}<noinclude>\n[[Category:Wt/kbd]]\n</noinclude>";

    result
}

pub fn main() {
    let possible_templates = [
        "спр-лъэмыӏ-0-0д-э",
        "спр-лъэмыӏ-0-0д-ы",
        "спр-лъэмыӏ-0-0л-ы",
        "спр-лъэмыӏ-0-0т-ы",
        "спр-лъэмыӏ-0-бд-э",
        "спр-лъэмыӏ-0-бдэа-э",
        "спр-лъэмыӏ-0-бт-ы",
        "спр-лъэмыӏ-0-бй-ы",
        "спр-лъэӏ-0-дблэа-ы",
        "спр-лъэӏ-0-дбд-ы",
        "спр-лъэӏ-0-жь0й-ы",
        "спр-лъэӏ-0-д0д-э",
        "спр-лъэӏ-0-убт-ы",
        "спр-лъэӏ-0-д0д-ы",
        "спр-лъэӏ-0-д0л-ы",
        // "спр-лъэмыӏ-е-бд-ы",
    ];

    // those are only test roots so that one can visually test the tables better.
    // In many cases the resulting table won't correspond to real words.
    let mut test_roots: std::collections::HashMap<&str, &str>;
    test_roots = std::collections::HashMap::new();
    test_roots.insert("0д", "в");
    test_roots.insert("0л", "гъу");
    test_roots.insert("0т", "гъ");

    test_roots.insert("бд", "гупсыс");
    test_roots.insert("бдэа", "лэжь");
    test_roots.insert("бт", "дыхьэшх");
    test_roots.insert("бй", "же");

    test_roots.insert("дблэа", "лъагъу");
    test_roots.insert("дбд", "тхьэщI");
    test_roots.insert("жь0й", "и");
    test_roots.insert("д0д", "щI");
    test_roots.insert("убт", "ух");
    test_roots.insert("д0д", "хь");
    test_roots.insert("д0л", "ху");

    // спр-лъэӏ-зэхэ-д0д-ы
    let template = "спр-лъэӏ-зэхэ-д0д-ы"; // tr. base. vl. e.g. хьын
                                        // let template = "спр-лъэмыӏ-0-0д-ы"; // intr. base. vl. e.g. плъэн
    let template_desc = template::create_template_from_string(template.to_owned()).unwrap();
    let template_str = create_template(template_desc);

    if let Some(root) = test_roots.get(template::get_root_str(template)) {
        let result = template_str.replace("{{{псалъэпкъ}}}", root);
        println!("{}", result);
    } else {
        println!("{}", template_str);
    }
}
