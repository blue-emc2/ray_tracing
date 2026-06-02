mod vec3;
use vec3::Vec3;

fn main() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);

    println!("a + b = {:?}", a + b); // Vec3 { x: 5.0, y: 7.0, z: 9.0 }
    println!("a - b = {:?}", a - b); // Vec3 { x: -3.0, y: -3.0, z: -3.0 }
    println!("-a   = {:?}", -a); // Vec3 { x: -1.0, y: -2.0, z: -3.0 }
}
