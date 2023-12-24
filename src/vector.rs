use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
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
            x,
            y,
            magnitude: Vector2::calculate_magnitude(x, y),
        }
    }

    fn from_zero() -> Vector2 {
        Vector2 {
            x: 0.,
            y: 0.,
            magnitude: 0.,
        }
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
        self.magnitude
    }

    pub fn get_components(&self) -> Components {
        Components::new(self.get_x(), self.get_y(), self.get_magnitude())
    }

    /// result: (x, y, magnitude)
    pub fn get_components_tuple(&self) -> (f64, f64, f64) {
        (self.get_x(), self.get_y(), self.get_magnitude())
    }

    // Set component
    /// Magnitude gets calculated automatically
    pub fn set_components(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.magnitude = Vector2::calculate_magnitude(x, y)
    }
    /// Magnitude gets calculated automatically
    pub fn set_x(&mut self, x: f64) {
        self.set_components(x, self.get_y());
    }
    /// Magnitude gets calculated automatically
    pub fn set_y(&mut self, y: f64) {
        self.set_components(self.get_x(), y);
    }

    /// This shortens the vector but keeps the direction the same.
    /// X and Y will be modified
    pub fn set_magnitude(&mut self, s: f64) {
        if self.get_magnitude() == 0. {
            self.add_scalar(1.);
        }

        self.multiply_scalar(s / self.get_magnitude());
    }

    pub fn set_magnitude_vector(&self, s: f64) -> Vector2 {
        let mut v = self.clone();
        if self.get_magnitude() == 0. {
            v = self.add_scalar_vector(1.);
        }

        v.multiply_scalar_vector(s / v.get_magnitude())
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
        self.set_components(self.get_x() - s, self.get_y() - s);
    }

    pub fn subtract_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::from_components(self.get_x() - v.get_x(), self.get_y() - v.get_y())
    }

    pub fn subtract_scalar_vector(&self, s: f64) -> Vector2 {
        Vector2::from_components(self.get_x() - s, self.get_y() - s)
    }

    pub fn multiply(&mut self, v: &Vector2) {
        self.multiply_scalar_x(v.get_x());
        self.multiply_scalar_y(v.get_y());
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

    /// Returns -1 if one of the magnitudes equals 0
    pub fn angle(&self, v: &Vector2) -> f64 {
        if self.get_magnitude() == 0. || v.get_magnitude() == 0. {
            return -1.;
        }

        (Vector2::dot_product(self, v) / self.get_magnitude() * v.get_magnitude()).cos() * -1.
    }

    pub fn dot(&self, v: &Vector2) -> f64 {
        Vector2::dot_product(self, v)
    }

    fn dot_product(v1: &Vector2, v2: &Vector2) -> f64 {
        v1.get_x() * v2.get_x() + v1.get_y() * v2.get_y()
    }

    pub fn normalize(&mut self) {
        let v = Vector2::normalize_vector(&self);
        self.set_components(v.get_x(), v.get_y());
    }

    pub fn normalized(&self) -> Vector2 {
        Vector2::normalize_vector(&self)
    }

    /// If magnitude equals 0 a zero vector will be returned
    fn normalize_vector(v: &Vector2) -> Vector2 {
        if v.get_magnitude() == 0. {
            return Vector2::from_zero();
        }

        Vector2::from_components(v.get_x() / v.get_magnitude(), v.get_y() / v.get_magnitude())
    }

    pub fn get_distance_scalar(&self, v: &Vector2) -> f64 {
        Vector2::get_distance(&self, v).get_magnitude()
    }

    pub fn get_distance_vector(&self, v: &Vector2) -> Vector2 {
        Vector2::get_distance(&self, v)
    }

    fn get_distance(v1: &Vector2, v2: &Vector2) -> Vector2 {
        v1.subtract_vector(v2)
    }

    pub fn reverse(&mut self) {
        self.multiply_scalar(-1.);
    }

    pub fn reverse_vector(&self) -> Vector2 {
        self.multiply_scalar_vector(-1.)
    }

    fn calculate_magnitude(x: f64, y: f64) -> f64 {
        (x.abs().powf(2.) + y.abs().powf(2.)).sqrt()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_new() {
        let v = Vector2::new();

        assert_eq!((0., 0., 0.), (v.get_x(), v.get_y(), v.get_magnitude()))
    }

    #[test]
    fn vector_from_components() {
        let v = Vector2::from_components(2., 5.);

        assert_eq!((2., 5., 5.385164807134504), v.get_components_tuple())
    }

    #[test]
    fn get_x() {
        let v = Vector2::from_components(5., 2.);

        assert_eq!(5., v.get_x())
    }

    #[test]
    fn get_y() {
        let v = Vector2::from_components(5., 2.);

        assert_eq!(2., v.get_y())
    }

    #[test]
    fn get_magnitude() {
        let v = Vector2::from_components(2., 5.);

        assert_eq!(5.385164807134504, v.get_magnitude())
    }

    #[test]
    fn get_components() {
        let v = Vector2::from_components(2., 5.);

        assert_eq!(
            Components::new(2., 5., 5.385164807134504),
            v.get_components()
        )
    }

    #[test]
    fn set_components() {
        let mut v = Vector2::new();
        v.set_components(1., 2.);

        assert_eq!((1., 2.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn set_x() {
        let mut v = Vector2::new();
        v.set_x(5.);

        assert_eq!(5., v.get_x())
    }

    #[test]
    fn set_y() {
        let mut v = Vector2::new();
        v.set_y(2.);

        assert_eq!(2., v.get_y())
    }

    #[test]
    fn set_magnitude() {
        let mut v = Vector2::new();
        v.set_magnitude(4.);

        assert_eq!(4., v.get_magnitude().ceil())
    }

    #[test]
    fn set_magnitude_vector() {
        let v = Vector2::new();
        let v = v.set_magnitude_vector(4.);

        assert_eq!(4., v.get_magnitude().ceil())
    }

    #[test]
    fn add() {
        let mut v = Vector2::new();
        v.add(&Vector2::from_components(5., 6.));

        assert_eq!((5., 6.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn add_scalar() {
        let mut v = Vector2::new();
        v.add_scalar(5.);

        assert_eq!((5., 5.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn add_scalar_vector() {
        let v = Vector2::new();
        let v = v.add_scalar_vector(3.);

        assert_eq!((3., 3.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn add_vector() {
        let v = Vector2::new();
        let v = v.add_vector(&Vector2::from_components(2., 5.));

        assert_eq!((2., 5.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn subtract() {
        let mut v = Vector2::new();
        v.subtract(&Vector2::from_components(21., 5.));

        assert_eq!((-21., -5.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn subtract_scalar() {
        let mut v = Vector2::new();
        v.subtract_scalar(3.);

        assert_eq!((-3., -3.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn subtract_scalar_vector() {
        let v = Vector2::new();
        let v = v.subtract_scalar_vector(5.);

        assert_eq!((-5., -5.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn subtract_vector() {
        let v = Vector2::new();
        let v = v.subtract_vector(&Vector2::from_components(2., 1.));

        assert_eq!((-2., -1.), (v.get_x(), v.get_y()))
    }

    #[test]
    fn multiply() {
        let mut v = Vector2::from_components(5., 5.);
        v.multiply(&Vector2::from_components(2., 2.));

        assert_eq!((10., 10.), (v.get_x(), v.get_y()))
    }

    // add mor unit tests
}
