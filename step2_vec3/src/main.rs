mod vec3;
use vec3::Vec3;

fn main() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);

    println!("a + b = {:?}", a + b); // Vec3 { x: 5.0, y: 7.0, z: 9.0 }
    println!("a - b = {:?}", a - b); // Vec3 { x: -3.0, y: -3.0, z: -3.0 }
    println!("-a   = {:?}", -a); // Vec3 { x: -1.0, y: -2.0, z: -3.0 }

    let a = Vec3::new(1.0, 2.0, 3.0);

    println!("a * 2.0 = {:?}", a * 2.0); // Vec3 { x: 2.0, y: 4.0, z: 6.0 }
    println!("2.0 * a = {:?}", 2.0 * a); // Vec3 { x: 2.0, y: 4.0, z: 6.0 }

    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);

    println!("a / 2.0       = {:?}", a / 2.0);
    println!("a * b         = {:?}", a * b); // (4, 10, 18)
    println!("a.dot(b)      = {}", a.dot(b)); // 32
    println!("a.cross(b)    = {:?}", a.cross(b)); // (-3, 6, -3)
    println!("a.length_sq() = {}", a.length_squared()); // 14
    println!("a.length()    = {}", a.length()); // 3.7416...

    let unit = Vec3::new(3.0, 0.0, 0.0).unit_vector();
    println!("unit_vector   = {:?}", unit); // (1, 0, 0)
    println!("|unit|        = {}", unit.length()); // 1.0
}
