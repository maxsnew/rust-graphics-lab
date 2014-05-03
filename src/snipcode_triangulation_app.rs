
// Extern crates.
use piston::*;
use graphics::*;

// Local crate.
use test_polygons;
use conversion;
use snipcode_triangulation;

pub struct App;

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        let polygon = test_polygons::SQUARE_CLOCKWISE;
        let polygon = conversion::to_vec_vector2d(polygon.data);
        let triangles = snipcode_triangulation::process(polygon.as_slice());
        let triangles = triangles.unwrap();
        let triangles = conversion::to_vec_f64(triangles.as_slice());
        let n = triangles.len() / (3 * 2);

        // TEST
        // c.rect(0.0, 0.0, 1.0, 1.0).rgba(1.0, 0.0, 0.0, 1.0).fill(gl);

        for i in range(0, n) {
            let start = i * 3 * 2;
            let end = (i + 1) * 3 * 2;
            c.polygon(triangles.slice(start, end))
            .rgba(1.0, 0.0, 0.0, 1.0).fill(gl);
        }
    }
}

impl App {
    pub fn new() -> App {
        App
    }
}

