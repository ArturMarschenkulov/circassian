/// This module will contain everything specific to wikipsalhalhe. In the future this may also include the normal wiki.
///
///
mod table;
pub mod template;

use crate::{
    evaluation,
    morpho::{self, Case, Number, Person, PersonMarker, Polarity, Pronoun, Tense},
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

    for polarity in Polarity::variants() {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ:", polarity.to_string_prefix()));

        let morphemes = morpho::new_masdar(&polarity, &desc.preverb, &desc.stem);
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

    Pronoun::variants_case(&desc.transitivity.subject_case())
        .iter()
        .map(|pronoun| pronoun.to_string())
        .for_each(|pronoun| table.add(pronoun));

    let number_and_person = Number::variants_iter()
        .flat_map(|n| Person::variants_iter().map(move |p| (n, p)))
        .collect::<Vec<_>>();

    for polarity in Polarity::variants() {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity.to_string_prefix()));

        for (number, person) in &number_and_person {
            let (abs_marker, erg_marker) = get_person_markers(
                &desc.transitivity.subject_case(),
                &person.clone(),
                &number.clone(),
            );

            let morphemes = morpho::new_masdar_personal(
                &polarity,
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
    Pronoun::variants_person(&Person::Second)
        .iter()
        .map(|pronoun| pronoun.to_string())
        .for_each(|pronoun| table.add(pronoun));

    for polarity in Polarity::variants() {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity.to_string_prefix()));
        for number in &Number::variants() {
            let (abs_marker, erg_marker) =
                get_person_markers(&desc.transitivity.subject_case(), &Person::Second, number);

            let morphemes = morpho::new_imperative(
                &polarity,
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

fn get_person_markers(
    subject_case: &Case,
    person: &Person,
    number: &Number,
) -> (PersonMarker, Option<PersonMarker>) {
    if subject_case == &Case::Absolutive {
        (PersonMarker::new(*person, *number, Case::Absolutive), None)
    } else {
        (
            PersonMarker::new(Person::Third, *number, Case::Absolutive),
            Some(PersonMarker::new(*person, *number, Case::Ergative)),
        )
    }
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

    Pronoun::variants_case(&desc.transitivity.subject_case())
        .iter()
        .map(|pronoun| pronoun.to_string())
        .for_each(|pronoun| table.add(pronoun));

    let number_and_person = Number::variants_iter()
        .flat_map(|n| Person::variants_iter().map(move |p| (n, p)))
        .collect::<Vec<_>>();

    for polarity in Polarity::variants() {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity.to_string_prefix()));
        for (number, person) in &number_and_person {
            let morphemes =
                morpho::new_imperative_raj(&polarity, &desc.preverb, &desc.stem, person, number);
            let string = evaluation::evaluate_morphemes(&morphemes);
            table.add(string);
        }
    }
    table.to_string()
}

fn table_indicative(desc: &template::TemplateDesc) -> String {
    let mut table = Wikitable::new();
    table.add("ЗэраӀуатэ наклоненэ".to_owned());
    let subject_case = &desc.transitivity.subject_case();

    Pronoun::variants_case(&desc.transitivity.subject_case())
        .iter()
        .map(|pronoun| pronoun.to_string())
        .for_each(|pronoun| table.add(pronoun));

    let tense_and_polarity = Tense::variants_iter()
        .flat_map(|t| Polarity::variants_iter().map(move |p| (t, p)))
        .collect::<Vec<_>>();

    let number_and_person = Number::variants_iter()
        .flat_map(|n| Person::variants_iter().map(move |p| (n, p)))
        .collect::<Vec<_>>();

    for (tense, polarity) in tense_and_polarity {
        table.add_row();
        table.add(indcicative_tense_string(&tense, &polarity));
        for (number, person) in &number_and_person {
            let (abs_marker, erg_marker) = get_person_markers(subject_case, person, number);
            let morphemes = morpho::new_indicative(
                &polarity,
                &tense,
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

fn table_interrogative(desc: &template::TemplateDesc) -> String {
    let mut table = Wikitable::new();
    table.add("ЗэрыупщӀэ наклоненэ".to_owned());
    let subject_case = &desc.transitivity.subject_case();

    Pronoun::variants_case(&desc.transitivity.subject_case())
        .iter()
        .map(|pronoun| pronoun.to_string())
        .for_each(|pronoun| table.add(pronoun));

    let tense_and_polarity = Tense::variants_iter()
        .flat_map(|t| Polarity::variants_iter().map(move |p| (t, p)))
        .collect::<Vec<_>>();

    let number_and_person = Number::variants_iter()
        .flat_map(|n| Person::variants_iter().map(move |p| (n, p)))
        .collect::<Vec<_>>();

    for (tense, polarity) in tense_and_polarity {
        table.add_row();
        table.add(indcicative_tense_string(&tense, &polarity));
        for (number, person) in &number_and_person {
            let (abs_marker, erg_marker) = get_person_markers(subject_case, person, number);
            let morphemes = morpho::new_interrogative(
                &polarity,
                &tense,
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

fn table_conditional(desc: &template::TemplateDesc) -> String {
    let mut table = Wikitable::new();
    table.add("условнэ ипэжыпӀэкӀэ щыӀэ наклоненэ".to_owned());
    let subject_case = &desc.transitivity.subject_case();

    Pronoun::variants_case(&desc.transitivity.subject_case())
        .iter()
        .map(|pronoun| pronoun.to_string())
        .for_each(|pronoun| table.add(pronoun));

    let tense_and_polarity = Tense::variants_iter()
        .flat_map(|t| Polarity::variants_iter().map(move |p| (t, p)))
        .collect::<Vec<_>>();

    let number_and_person = Number::variants_iter()
        .flat_map(|n| Person::variants_iter().map(move |p| (n, p)))
        .collect::<Vec<_>>();

    for (tense, polarity) in tense_and_polarity {
        if ![
            Tense::Present,
            Tense::Perfect,
            Tense::FarPast1,
            Tense::Future1,
            Tense::Future2,
        ]
        .contains(&tense)
        {
            continue;
        }
        table.add_row();
        table.add(indcicative_tense_string(&tense, &polarity));
        for (number, person) in &number_and_person {
            let (abs_marker, erg_marker) = get_person_markers(subject_case, person, number);
            let morphemes = morpho::new_conditional(
                &polarity,
                &tense,
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

fn table_conditional_2(desc: &template::TemplateDesc) -> String {
    let mut table = Wikitable::new();
    table.add("условнэ ипэжыпӀэкӀэ щыӀэ наклоненэ".to_owned());
    let subject_case = &desc.transitivity.subject_case();

    Pronoun::variants_case(&desc.transitivity.subject_case())
        .iter()
        .map(|pronoun| pronoun.to_string())
        .for_each(|pronoun| table.add(pronoun));

    let tense_and_polarity = Tense::variants_iter()
        .flat_map(|t| Polarity::variants_iter().map(move |p| (t, p)))
        .collect::<Vec<_>>();

    let number_and_person = Number::variants_iter()
        .flat_map(|n| Person::variants_iter().map(move |p| (n, p)))
        .collect::<Vec<_>>();

    for (tense, polarity) in tense_and_polarity {
        if ![
            Tense::Present,
            Tense::Perfect,
            Tense::FarPast1,
            Tense::Future1,
            Tense::Future2,
        ]
        .contains(&tense)
        {
            continue;
        }
        table.add_row();
        table.add(indcicative_tense_string(&tense, &polarity));
        for (number, person) in &number_and_person {
            let (abs_marker, erg_marker) = get_person_markers(subject_case, person, number);
            let morphemes = morpho::new_conditional_2(
                &polarity,
                &tense,
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

fn indcicative_tense_string(tense: &Tense, polarity: &Polarity) -> String {
    let p = polarity.to_string_prefix();
    match tense {
        Tense::Present => format!("ит зэман – щы{}Ӏэныгъэ:", p),
        Tense::Imperfect => format!("блэкӀа зэфӀэмыкӀа зэман (имперфект) – щы{}Ӏэныгъэ:", p),
        Tense::Perfect => format!("блэкӀа зэфӀэкӀа зэман (перфект) – щы{}Ӏэныгъэ:", p),
        Tense::PlusQuamPerfect => format!(
            "блэкӀа зэфӀэкӀа и пэ ит зэман (плюс квамперфект) – щы{}Ӏэныгъэ:",
            p
        ),
        Tense::FarPast1 => format!("блэкӀа жыжьэ зэфӀэкӀа зэман – щы{}Ӏэныгъэ:", p),
        Tense::FarPast2 => format!("блэкӀа жыжьэ зэфӀэкӀа и пэ ит зэман – щы{}Ӏэныгъэ:", p),
        Tense::Aorist1 => format!("блэкӀа гъэкӀэщӀа зэман I – щы{}Ӏэныгъэ:", p),
        Tense::Aorist2 => format!("блэкӀа гъэкӀэщӀа зэман II – щы{}Ӏэныгъэ:", p),
        Tense::Future1 => format!("къэкӀуэну мыгъэбелджыла зэман – щы{}Ӏэныгъэ:", p),
        Tense::Future2 => format!("къэкӀуэну гъэбелджыла зэман – щы{}Ӏэныгъэ:", p),
    }
}
fn create_template(desc: template::TemplateDesc) -> String {
    let mut result = "".to_string();
    result += &format!("<!-- Template:Wt/kbd/{} -->\n", desc.original_string);
    let r = vec![
        // Инфинитив (масдар)
        table_masdar(&desc),
        // Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа
        table_masdar_personal(&desc),
        // унафэ наклоненэ
        table_imperative(&desc),
        // Ре-кӀэ унафэ наклоненэ
        table_imperative_raj(&desc),
        // This part needs mostly the same logic, as only the endings change.
        table_indicative(&desc),
        table_interrogative(&desc),
        table_conditional(&desc),
        table_conditional_2(&desc),
    ];

    for table in r {
        result += &table;
        result += "\n----------------------------------------------\n";
    }
    // result += &r.join("\n-\n");

    result += "|}<noinclude>\n[[Category:Wt/kbd]]\n</noinclude>";

    result
}

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
pub fn main() {
    let _possible_templates = [
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
    test_roots.insert("жь0д", "гъ");
    test_roots.insert("д0д", "щI");
    test_roots.insert("убт", "ух");
    test_roots.insert("д0д", "х");
    test_roots.insert("д0л", "ху");

    // спр-лъэӏ-зэхэ-д0д-ы
    let template = "спр-лъэмыӏ-0-д0д-ы"; // tr. base. vl. e.g. хьын
                                         // let template = "спр-лъэмыӏ-0-0д-ы"; // intr. base. vl. e.g. плъэн
    let template_desc = template::create_template_from_string(template.to_owned()).unwrap();
    let template_str = create_template(template_desc);

    if let Some(root) = test_roots.get(template::root_str(template)) {
        let result = template_str.replace("{{{псалъэпкъ}}}", root);
        println!("{}", result);
    } else {
        println!("{}", template_str);
    }
}
