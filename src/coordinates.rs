use bevy::math::DVec3;

pub trait ToPolar {
    fn polar(self) -> Polar;
}

impl ToPolar for DVec3 {
    fn polar(self) -> Polar {
        return Polar::new(-self.x, -self.y, -self.z);
    }
}

pub struct Polar {
    pub longitude: f64,
    pub altitude: f64,
    pub latitude: f64,
}

impl Polar {
    pub fn new(longitude: f64, altitude: f64, latitude: f64) -> Self {
        Polar {
            longitude: longitude,
            altitude: altitude,
            latitude: latitude,
        }
    }
}

pub trait ToCartesian {
    fn cartesian(self) -> DVec3;
}

impl ToCartesian for Polar {
    fn cartesian(self) -> DVec3 {
        return DVec3::new(-self.longitude, -self.altitude, -self.latitude);
    }
}
