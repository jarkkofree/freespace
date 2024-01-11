pub fn get_coord (translation: Vec3) -> String {
    let x = translation.x;
    let y = translation.y;
    let z = translation.z;

    // convert decimal to scientific notation
    let x = format!("{:e}", x);
    let y = format!("{:e}", y);
    let z = format!("{:e}", z);

    format!("{} {} {}", x, y, z)
}