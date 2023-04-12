use nannou::draw::properties::color;
use nannou::prelude::*;

use cavalier_contours::pline_open;
use cavalier_contours::polyline::*;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {

    let draw = app.draw();
    draw.background().color(BLACK);

    let mut rng = rand::thread_rng();

    let polyLine_vertex_list = generate_vertex_list(10);

    // Create a new polyline
    let mut polyline = Polyline::with_capacity(polyLine_vertex_list.len(), false);

    // Add the vertices from the generated list
    for (x, y, bulge) in polyLine_vertex_list {
        polyline.add_vertex(PlineVertex::new(x, y, bulge));
    }

    polyline.parallel_offset(50.0);



    let polyline_vertex_data = &polyline.vertex_data;

    


    let vec2_points = vertexData_to_vec2List(polyline_vertex_data);

    // draw line from points
    draw.polyline()
        .weight(3.0)
        .points(vec2_points)
        .color(RED);
        // .points_colored(pointss);

    draw.to_frame(app, &frame).unwrap();
}

fn vertexData_to_vec2List(vertex_data: &[PlineVertex<f64>]) -> Vec<Vec2> {
    let mut vec2_points: Vec<Vec2> = Vec::new();

    for vertex in vertex_data {
        let x = vertex.x as f32;
        let y = vertex.y as f32;
        vec2_points.push(pt2(x, y));
    }

    vec2_points
}

fn generate_vertex_list(num_vertices: usize) -> Vec<(f64, f64, f64)> {
        
    // let mut rng = rand::thread_rng();
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    let range: f64 = 100.0;

    let vertex_list: Vec<(f64, f64, f64)> = (0..num_vertices)
        .map(|_| {
            let x: f64 = rng.gen_range(-range..range); // Adjust the range as needed
            let y: f64 = rng.gen_range(-range..range); // Adjust the range as needed
            let bulge = 0.0;
            (x, y, bulge)
        })
        .collect();

    vertex_list
}


