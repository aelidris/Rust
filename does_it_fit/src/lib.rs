mod areas_volumes;
pub use areas_volumes::*;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    match kind {
        GeometricalShapes::Rectangle => {
            let surface = rectangle_area(a, b);
            times * surface <= x * y
        }
        GeometricalShapes::Triangle => {
            let surface = triangle_area(a, b);
            (times as f64) * surface <= ((x * y) as f64)
        }
        GeometricalShapes::Square => {
            let surface = square_area(a);
            times * surface <= x * y
        }
        GeometricalShapes::Circle => {
            let surface = circle_area(a);
            (times as f64) * surface <= ((x * y) as f64)
        }
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    let container_volume = x * y * z;

    match kind {
        GeometricalVolumes::Cube => {
            let volume = cube_volume(a);
            times * volume <= container_volume
        }
        GeometricalVolumes::Sphere => {
            let volume = sphere_volume(a) as f64;
            (times as f64) * volume <= (container_volume as f64)
        }
        GeometricalVolumes::Cone => {
            let volume = cone_volume(a, b) as f64;
            (times as f64) * volume <= (container_volume as f64)
        }
        GeometricalVolumes::TriangularPyramid => {
            let volume = triangular_pyramid_volume(a as f64, b) as f64;
            (times as f64) * volume <= (container_volume as f64)
        }
        GeometricalVolumes::Parallelepiped => {
            let volume = parallelepiped_volume(a, b, c);
            times * volume <= container_volume
        }
    }
}
