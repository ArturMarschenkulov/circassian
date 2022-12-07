pub struct Wikitable {
    cells: Vec<Vec<String>>,
}
impl Wikitable {
    pub fn new() -> Self {
        Wikitable {
            cells: vec![vec![]],
        }
    }
    pub fn add_row(&mut self) {
        self.cells.push(vec![]);
    }
    pub fn add(&mut self, s: String) {
        let last_row = self.cells.last_mut().unwrap();
        last_row.push(s);
    }
}

impl std::fmt::Display for Wikitable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        result += "{| class=\"wikitable\"\n";
        result += "|-\n";
        result += &format!("! {} ", self.cells[0][0]);
        for i in 1..self.cells[0].len() {
            result += &format!("!! {} ", self.cells[0][i]);
        }
        result += "\n";
        result += "|-\n";

        for i in 1..self.cells.len() {
            result += &format!("| {} ", self.cells[i][0]);
            for j in 1..self.cells[i].len() {
                result += &format!("|| {} ", self.cells[i][j]);
            }
            result += "\n";
            result += "|-\n";
        }
        result += "|}";
        write!(f, "{}", result)
    }
}
