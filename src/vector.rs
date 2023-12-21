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
    magnitude: f64,
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
        Vector2 {
            magnitude: Vector2::calculate_magnitude(x, y),
            x,
            y,
        }
    }

    fn from_zero() -> Vector2 {
        Vector2 {
            magnitude: 0.,
            x: 0.,
            y: 0.,
        }
    }

    // Get component
    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_magnitude(&self) -> f64 {
        self.magnitude
    }

    /// return (x, y, magnitude)
    pub fn get_components(&self) -> Components {
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
        self.update_magnitude();
    }
    /// Magnitude gets calculated automatically
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
        self.update_magnitude();
    }

    // Calculations
    pub fn add(&mut self, v: &Vector2) {
        self.set_components(self.x + v.x, self.y + v.y);
    }

    pub fn add_scalar(&mut self, s: f64) {
        self.set_components(self.x + s, self.y + s);
    }

    pub fn add_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::from_components(self.x + v.x, self.y + v.y)
    }

    pub fn add_scalar_vector(&self, s: f64) -> Vector2 {
        Vector2::from_components(self.x + s, self.y + s)
    }

    pub fn subtract(&mut self, v: &Vector2) {
        self.set_components(self.x - v.x, self.y - v.y);
    }

    pub fn subtract_scalar(&mut self, s: f64) {
        self.set_components(self.x + s, self.y + s);
    }

    pub fn subtract_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::from_components(self.x - v.x, self.y - v.y)
    }

    pub fn subtract_scalar_vector(&self, s: f64) -> Vector2 {
        Vector2::from_components(self.x + s, self.y + s)
    }

    pub fn multiply_scalar(&mut self, s: f64) {
        self.set_components(s * self.x, s * self.y)
    }

    pub fn multiply_scalar_x(&mut self, s: f64) {
        self.set_x(self.x * s);
    }

    pub fn multiply_scalar_y(&mut self, s: f64) {
        self.set_y(self.y * s);
    }

    pub fn multiply_scalar_vector(&self, s: f64) -> Vector2 {
        Vector2::from_components(s * self.x, s * self.y)
    }

    pub fn multiply_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::from_components(self.x * v.x, self.y * v.y)
    }

    pub fn multiply_dot(&self, v: &Vector2) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn angle(&self, v: &Vector2) -> f64 {
        (Vector2::dot_product(self, v) / self.magnitude * v.magnitude).cos() * -1.
    }

    pub fn dot(&self, v: &Vector2) -> f64 {
        Vector2::dot_product(self, v)
    }

    fn dot_product(v1: &Vector2, v2: &Vector2) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    pub fn normalized(&self) -> Vector2 {
        Vector2::normalize_vector(&self)
    }

    pub fn normalize(&mut self) {
        let v = Vector2::normalize_vector(&self);
        self.set_components(v.x, v.y);
    }

    pub fn get_distance(&self, v: Vector2) -> f64 {
        self.subtract_vector(&v).get_magnitude()
    }

    fn normalize_vector(v: &Vector2) -> Vector2 {
        Vector2::from_components(v.x / v.magnitude, v.y / v.magnitude)
    }

    fn calculate_magnitude(x: f64, y: f64) -> f64 {
        (x.powf(2.) + y.powf(2.)).sqrt()
    }

    fn update_magnitude(&mut self) {
        self.magnitude = Vector2::calculate_magnitude(self.x, self.y);
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x: {}; y: {}; magnitude: {};",
            self.x, self.y, self.magnitude
        )
    }
}
