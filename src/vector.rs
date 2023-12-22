use core::fmt;

#[derive(Clone, Copy)]
pub struct Components {
    pub magnitude: f64,
    pub x: f64,
    pub y: f64,
}
impl Components {
    pub fn new(x: f64, y: f64, magnitude: f64) -> Components {
        Components { magnitude, x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Vector2 {
    x: f64,
    y: f64,
}
impl Vector2 {
    /// Create a new zero Vector
    pub fn new() -> Vector2 {
        Vector2::from_zero()
    }

    /// Create Vector from x and y values
    pub fn from_components(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    fn from_zero() -> Vector2 {
        Vector2 { x: 0., y: 0. }
    }

    // Get component
    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    /// Calculates and returns magnitude
    pub fn get_magnitude(&self) -> f64 {
        Vector2::calculate_magnitude(self.get_x(), self.get_y())
    }

    /// return (x, y, magnitude)
    pub fn get_components(&mut self) -> Components {
        Components::new(self.get_x(), self.get_y(), self.get_magnitude())
    }

    // Set component
    /// Magnitude gets calculated automatically
    pub fn set_components(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }
    /// Magnitude gets calculated automatically
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    /// Magnitude gets calculated automatically
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// This shortens the vector but keeps the direction the same.
    /// X and Y will be modified
    pub fn set_magnitude(&mut self, s: f64) {
        self.multiply_scalar(s / self.get_magnitude());
    }

    pub fn set_magnitude_vector(&self, s: f64) -> Vector2 {
        self.multiply_scalar_vector(s / self.get_magnitude())
    }

    // Calculations
    pub fn add(&mut self, v: &Vector2) {
        self.set_components(self.get_x() + v.get_x(), self.get_y() + v.get_y());
    }

    pub fn add_scalar(&mut self, s: f64) {
        self.set_components(self.get_x() + s, self.get_y() + s);
    }

    pub fn add_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::from_components(self.get_x() + v.get_x(), self.get_y() + v.get_y())
    }

    pub fn add_scalar_vector(&self, s: f64) -> Vector2 {
        Vector2::from_components(self.get_x() + s, self.get_y() + s)
    }

    pub fn subtract(&mut self, v: &Vector2) {
        self.set_components(self.get_x() - v.get_x(), self.get_y() - v.get_y());
    }

    pub fn subtract_scalar(&mut self, s: f64) {
        self.set_components(self.get_x() + s, self.get_y() + s);
    }

    pub fn multiply(&mut self, v: &Vector2) {
        self.multiply_scalar_x(v.get_x());
        self.multiply_scalar_y(v.get_y());
    }

    pub fn subtract_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::from_components(self.get_x() - v.get_x(), self.get_y() - v.get_y())
    }

    pub fn subtract_scalar_vector(&self, s: f64) -> Vector2 {
        Vector2::from_components(self.get_x() + s, self.get_y() + s)
    }

    pub fn multiply_scalar(&mut self, s: f64) {
        self.set_components(self.get_x() * s, self.get_y() * s)
    }

    pub fn multiply_scalar_x(&mut self, s: f64) {
        self.set_x(self.get_x() * s);
    }

    pub fn multiply_scalar_y(&mut self, s: f64) {
        self.set_y(self.get_y() * s);
    }

    pub fn multiply_scalar_vector(&self, s: f64) -> Vector2 {
        Vector2::from_components(self.get_x() * s, self.get_y() * s)
    }

    pub fn multiply_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::from_components(self.get_x() * v.get_x(), self.get_y() * v.get_y())
    }

    pub fn multiply_dot(&self, v: &Vector2) -> f64 {
        self.get_x() * v.get_x() + self.get_y() * v.get_y()
    }

    pub fn angle(&self, v: &Vector2) -> f64 {
        (Vector2::dot_product(self, v) / self.get_magnitude() * v.get_magnitude()).cos() * -1.
    }

    pub fn dot(&self, v: &Vector2) -> f64 {
        Vector2::dot_product(self, v)
    }

    fn dot_product(v1: &Vector2, v2: &Vector2) -> f64 {
        v1.get_x() * v2.get_x() + v1.get_y() * v2.get_y()
    }

    pub fn normalized(&self) -> Vector2 {
        Vector2::normalize_vector(&self)
    }

    pub fn normalize(&mut self) {
        let v = Vector2::normalize_vector(&self);
        self.set_components(v.get_x(), v.get_y());
    }

    pub fn get_distance(&self, v: Vector2) -> f64 {
        self.subtract_vector(&v).get_magnitude()
    }

    fn normalize_vector(v: &Vector2) -> Vector2 {
        Vector2::from_components(v.get_x() / v.get_magnitude(), v.get_y() / v.get_magnitude())
    }

    fn calculate_magnitude(x: f64, y: f64) -> f64 {
        (x.powf(2.) + y.powf(2.)).sqrt()
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x: {}; y: {}; magnitude: {};",
            self.x,
            self.y,
            self.get_magnitude(),
        )
    }
}
