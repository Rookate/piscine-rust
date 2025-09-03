mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area = rectangle_area(x, y);

    match kind {
        GeometricalShapes::Square => {
            if square_area(a) * times > area {
                return false;
            }
            true
        }
        GeometricalShapes::Circle => {
            if circle_area(a) * times as f64 > area as f64 {
                return false;
            }
            true
        }
        GeometricalShapes::Rectangle => {
            if rectangle_area(a, b) * times > area {
                return false;
            }
            true
        }
        GeometricalShapes::Triangle => {
            if triangle_area(a, b) * times as f64 > area as f64 {
                return false;
            }
            true
        }
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let volume = parallelepiped_volume(x, y, z);

    match kind {
        GeometricalVolumes::Cube => {
            if cube_volume(a) * times > volume {
                return false;
            }
            true
        }
        GeometricalVolumes::Sphere => {
            if sphere_volume(a) * times as f64 > volume as f64 {
                return false;
            }
            true
        }
        GeometricalVolumes::Cone => {
            if cone_volume(a, b) * times as f64 > volume as f64 {
                return false;
            }
            true
        }
        GeometricalVolumes::TriangularPyramid => {
            if triangular_pyramid_volume(a as f64, b) * times as f64 > volume as f64 {
                return false;
            }
            true
        }
        GeometricalVolumes::Parallelepiped => {
            if parallelepiped_volume(a, b, c) * times > volume {
                return false;
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
            area_fit((2, 4), GeometricalShapes::Rectangle, 100, (2, 1))
        );
        println!(
            "Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
            area_fit((5, 5), GeometricalShapes::Triangle, 3, (5, 3))
        );
        println!(
            "Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
            volume_fit((5, 5, 5), GeometricalVolumes::Sphere, 3, (2, 0, 0))
        );
        println!(
            "Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
            volume_fit((5, 7, 5), GeometricalVolumes::Parallelepiped, 1, (6, 7, 4))
        );
    }
}
