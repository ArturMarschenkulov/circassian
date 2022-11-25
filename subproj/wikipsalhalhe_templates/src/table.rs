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
    pub fn to_string(&self) -> String {
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
        return result.to_owned();
    }
}
