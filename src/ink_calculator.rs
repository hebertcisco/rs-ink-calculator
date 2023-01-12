pub(crate) struct Calculator {
    pub wall_area: f64,
    pub ceiling_area: f64,
}
impl Calculator {
    pub fn calculate_wall_area(&mut self, height: f64, width: f64, depth: f64) {
        self.wall_area = 2.0 * (width + depth) * height;
    }

    pub fn calculate_ceiling_area(&mut self, width: f64, depth: f64) {
        self.ceiling_area = width * depth;
    }

    pub fn calculate_required_literacy(&self) -> f64 {
        (self.wall_area + self.ceiling_area) / 10.0
    }
}
