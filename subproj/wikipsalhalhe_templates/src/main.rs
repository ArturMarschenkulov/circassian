// спр-лъэмыӏ-0-0д-э

enum PersonMarkerCase {
    // (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    // (-м) subject of transitive verb
    Ergative,
    // (-м) indirect object of intransitive and transitive verbs.
    Oblique,
}
#[derive(Clone, Copy, PartialEq)]
enum Number {
    Singular,
    Plural,
}
#[derive(Clone, Copy, PartialEq)]
enum Person {
    First,
    Second,
    Third,
}
enum SoundForm {
    Base,
    BeforeUnvoiced,
    BeforeVoiced,
    BeforeVowel,
}
struct PersonMarker {
    person: Person,
    number: Number,
    case: PersonMarkerCase,
    form: SoundForm,
}
impl PersonMarker {
    fn to_string(&self) -> String {
        let base_consonant = match (&self.person, &self.number) {
            (Person::First, Number::Singular) => "с",
            (Person::Second, Number::Singular) => "у",
            (Person::First, Number::Plural) => "д",
            (Person::Second, Number::Plural) => "ф",
            (Person::Third, Number::Singular) => "",
            (Person::Third, Number::Plural) => "",
        }
        .to_owned();
        let vowel = match &self.case {
            PersonMarkerCase::Absolutive => "ы",
            PersonMarkerCase::Ergative => "",
            PersonMarkerCase::Oblique => "э",
        }
        .to_owned();
        let s = base_consonant + &vowel;
        let s = if s == "уы" { "у".to_owned() } else { s };
        let s = if s == "ы" { "".to_owned() } else { s };
        let s = if s == "" { "и".to_owned() } else { s };
        return s.to_owned();
    }
}

struct Wikitable {
    header: String,
    cells: Vec<Vec<String>>,
}
impl Wikitable {
    fn new() -> Self {
        Wikitable {
            header: "".to_owned(),
            cells: vec![vec![]],
        }
    }
    fn add_row(&mut self) -> () {
        self.cells.push(vec![]);
    }
    fn add(&mut self, s: String) -> () {
        let last_row = self.cells.last_mut().unwrap();
        last_row.push(s);
    }
    fn to_string(&self) -> String {
        let mut result = String::new();
        result += &"{| class=\"wikitable\"\n";
        result += &"|-\n";
        result += &format!("! {} ", self.cells[0][0]);
        for i in 1..self.cells[0].len() {
            result += &format!("!! {} ", self.cells[0][i]);
        }
        result += &"\n";
        result += &"|-\n";

        for i in 1..self.cells.len() {
            result += &format!("| {} ", self.cells[i][0]);
            for j in 1..self.cells[i].len() {
                result += &format!("|| {} ", self.cells[i][j]);
            }
            result += &"\n";
            result += &"|-\n";
        }
        result += &"|}";
        return result.to_owned();
    }
}
enum Transitivity {
    Transitive,
    Intransitive,
}
enum Ending {
    A,
    Y,
}
impl std::fmt::Display for Ending {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Ending::A => write!(f, "э"),
            Ending::Y => write!(f, "ы"),
        }
    }
}
struct TemplateDesc {
    transitivity: Transitivity,
    preverb: String,
    ending: Ending,
    original_string: String,
}

fn get_masdar(desc: &TemplateDesc) -> String {
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
    let root = "{{{псалъэпкъ}}}".to_owned();
    let infinitve_ending = format!("{}н", &desc.ending);
    let negative_prefix = "мы";
    let table_name = "Инфинитив (масдар)".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name.clone());
    table.add("".to_owned());
    table.add_row();

    table.add("щыӀэныгъэ:".to_owned());
    table.add(format!("{}{}", root, infinitve_ending));
    table.add_row();

    table.add("щымыӀэныгъэ:".to_owned());
    table.add(format!("{}{}{}", negative_prefix, root, infinitve_ending));
    table.to_string()
}
fn get_masdar_personal(desc: &TemplateDesc) -> String {
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

    let root = "{{{псалъэпкъ}}}".to_owned();
    let infinitve_ending = format!("{}н", &desc.ending);
    let negative_prefix = "мы";

    let table_name = "Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name.clone());
    for pronoun in ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"].iter() {
        table.add(pronoun.to_string());
    }

    table.add_row();
    table.add("щыӀэныгъэ".to_owned());
    for number in vec![Number::Singular, Number::Plural] {
        for person in vec![Person::First, Person::Second, Person::Third] {
            let is_third_pl = number == Number::Plural && person == Person::Third;
            let marker = PersonMarker {
                person,
                number,
                case: PersonMarkerCase::Absolutive,
                form: SoundForm::Base,
            };
            let pl = if is_third_pl { "(хэ)" } else { "" };
            let s = format!(
                "{}{}{}",
                marker.to_string(),
                root,
                &(infinitve_ending.clone() + &pl)
            );
            table.add(s);
        }
    }

    table.add_row();
    table.add("щымыӀэныгъэ".to_owned());
    for number in vec![Number::Singular, Number::Plural] {
        for person in vec![Person::First, Person::Second, Person::Third] {
            let is_third_pl = number == Number::Plural && person == Person::Third;
            let marker = PersonMarker {
                person,
                number,
                case: PersonMarkerCase::Absolutive,
                form: SoundForm::Base,
            };
            let pl = if is_third_pl { "(хэ)" } else { "" };
            let s = format!(
                "{}{}{}{}",
                marker.to_string(),
                negative_prefix,
                root,
                &(infinitve_ending.clone() + &pl)
            );
            table.add(s);
        }
    }
    table.to_string()
}

fn get_imperative(desc: &TemplateDesc) -> String {
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
    let root = "{{{псалъэпкъ}}}".to_owned();
    let negative_prefix = "мы";

    let mut table = Wikitable::new();
    table.add("унафэ наклоненэ".to_owned());
    for pronoun in ["уэ", "фэ"].iter() {
        table.add(pronoun.to_string());
    }

    table.add_row();
    table.add("щыӀэныгъэ".to_owned());
    for number in vec![Number::Singular, Number::Plural] {
        let marker = PersonMarker {
            person: Person::Second,
            number,
            case: PersonMarkerCase::Absolutive,
            form: SoundForm::Base,
        };
        let s = marker.to_string();
        let s = if s == "у" { "".to_string() } else { s };

        let s = format!(" || {}{}{}", s, root, &desc.ending);
        table.add(s);
    }

    table.add_row();
    table.add("щымыӀэныгъэ".to_owned());
    for number in vec![Number::Singular, Number::Plural] {
        let marker = PersonMarker {
            person: Person::Second,
            number,
            case: PersonMarkerCase::Absolutive,
            form: SoundForm::Base,
        };
        let s = marker.to_string();

        let s = format!(" || {}мы{}{}", s, root, &desc.ending);
        table.add(s);
    }
    table.to_string()
}

fn get_imperative_raj(desc: &TemplateDesc) -> String {
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
    let root = "{{{псалъэпкъ}}}".to_owned();
    let infinitve_ending = format!("{}н", &desc.ending);
    let negative_prefix = "мы";
    let pronouns = "!! сэ  !! уэ !! ар !! дэ !! фэ !! ахэр";

    let mut table = Wikitable::new();
    table.add("Ре-кӀэ унафэ наклоненэ".to_owned());
    for pronoun in ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"].iter() {
        table.add(pronoun.to_string());
    }

    table.add_row();
    table.add("щыӀэныгъэ".to_owned());
    for number in vec![Number::Singular, Number::Plural] {
        for person in vec![Person::First, Person::Second, Person::Third] {
            let is_third_pl = number == Number::Plural && person == Person::Third;
            let marker = PersonMarker {
                person,
                number,
                case: PersonMarkerCase::Ergative,
                form: SoundForm::Base,
            };
            let s = format!(" || {}ре{}{}", marker.to_string(), root, &desc.ending);
            table.add(s);
        }
    }
    table.add("щымыӀэныгъэ".to_owned());
    for number in vec![Number::Singular, Number::Plural] {
        for person in vec![Person::First, Person::Second, Person::Third] {
            let marker = PersonMarker {
                person,
                number,
                case: PersonMarkerCase::Ergative,
                form: SoundForm::Base,
            };
            let s = format!(" || {}ремы{}{}", marker.to_string(), root, &desc.ending);
            table.add(s);
        }
    }
    table.to_string()
}
fn create_template(desc: TemplateDesc) -> String {
    // let root = "{{{псалъэпкъ}}}".to_owned();
    let mut result = "".to_string();
    result += &format!("<!-- Template:Wt/kbd/{} -->\n", desc.original_string);

    // let infinitve_ending = format!("{}н", &desc.ending);
    // let pronouns = "!! сэ  !! уэ !! ар !! дэ !! фэ !! ахэр";
    // let negative_prefix = "мы";

    // Инфинитив (масдар)
    result += &get_masdar(&desc);
    result += "\n-\n";

    // Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа
    result += &get_masdar_personal(&desc);
    result += "\n-\n";
    // унафэ наклоненэ
    result += &get_imperative(&desc);
    result += "\n-\n";

    // Ре-кӀэ унафэ наклоненэ
    result += &get_imperative_raj(&desc);
    result += "\n-\n";

    result += "|}<noinclude>\n[[Category:Wt/kbd]]\n</noinclude>";
    println!("{}", result);

    return result;
}
fn create_template_from_string(s: String) -> Option<String> {
    let ss = s.split("-").collect::<Vec<&str>>();
    println!("{:?}", ss);
    // Every string must start with "спр". If this is not the case, the string is false.
    if ss[0] != "спр" {
        println!("The string does not start with 'спр'");
        return None;
    }
    let transitivity = match ss[1] {
        "лъэмыӏ" => Transitivity::Intransitive,
        "лъэӏ" => Transitivity::Transitive,
        _ => {
            println!("The second string isn't either 'лъэмыӏ' or 'лъэӏ'");
            return None;
        }
    };
    let preverb = match ss[2] {
        "0" => None,
        _ => Some(ss[2]),
    };

    let ending = match ss.last() {
        Some(&"э") => Ending::A,
        Some(&"ы") => Ending::Y,
        _ => {
            println!("The last string isn't either 'э' or 'ы'");
            return None;
        }
    };
    let template_desc = TemplateDesc {
        transitivity: transitivity,
        preverb: preverb.unwrap_or(&"").to_owned(),
        ending: ending,
        original_string: s,
    };
    let template = create_template(template_desc);
    return Some(template);
}
fn main() {
    let template = "спр-лъэмыӏ-0-0д-э";
    let template = create_template_from_string(template.to_owned());
}
