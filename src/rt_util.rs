pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = f64::INFINITY;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}
