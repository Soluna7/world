use bevy::math::DVec3;
use coordinates::*;
mod coordinates;

fn main() {
    let position = DVec3::new(3.0, 5.0, 1.0);
    let position = position.polar();
    let longitude = position.longitude;
    let altitude = position.altitude;
    let latitude = position.latitude;
    println!("λ: {longitude}, α: {altitude}, ϕ: {latitude}");
    let position = position.cartesian();
    let x: f64 = position.x;
    let y: f64 = position.y;
    let z: f64 = position.z;
    println!("x: {x}, y: {y}, z: {z}");
}
