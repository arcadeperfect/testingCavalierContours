use nannou::draw::properties::color;
use nannou::prelude::*;

use cavalier_contours::pline_open;
use cavalier_contours::polyline::*;

use rand::Rng;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {

    let draw = app.draw();
    draw.background().color(BLACK);

    let mut rng = rand::thread_rng();

    let vertex_list = generate_vertex_list(10);

    // Create a new polyline
    let mut polyline = Polyline::with_capacity(vertex_list.len(), false);

    // Add the vertices from the generated list
    for (x, y, bulge) in vertex_list {
        polyline.add_vertex(PlineVertex::new(x, y, bulge));
    }


    let polyline_vertex_data = &polyline.vertex_data;
    let vec2_points = vertex_data_to_vec2(polyline_vertex_data);

    
    // let v = &polyline.vertex_data;

    // let mut offsettedPoints: Vec<Vec2> = Vec::new();

    // for i in 0..v.len() {
    //     let x = v[i].x as f32;
    //     let y = v[i].y as f32;
    //     let bulge = v[i].bulge;
    //     // println!("x: {}, y: {}, bulge: {}", x, y, bulge);
    //     // add to points
    //     offsettedPoints.push(pt2(x, y));
    // }
    
    draw.ellipse()
        .x_y(0.0, 0.0)
        .w_h(10.0, 10.0)
        .color(WHITE);  

    // draw line from points
    draw.polyline()
        .weight(3.0)
        .points(vec2_points)
        .color(RED);
        // .points_colored(pointss);

    // // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}


fn vertex_data_to_vec2(vertex_data: &[PlineVertex<f64>]) -> Vec<Vec2> {
    let mut vec2_points: Vec<Vec2> = Vec::new();

    for vertex in vertex_data {
        let x = vertex.x as f32;
        let y = vertex.y as f32;
        vec2_points.push(pt2(x, y));
    }

    vec2_points
}

fn generate_vertex_list(num_vertices: usize) -> Vec<(f64, f64, f64)> {
        let mut rng = rand::thread_rng();

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

fn randomPoints(amount: i32) -> Vec<Vec2> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<Vec2> = Vec::new();
    for _ in 0..amount {
        let x: f32 = rng.gen_range(-10.0..10.0); // Adjust the range as needed
        let y: f32 = rng.gen_range(-10.0..10.0); // Adjust the range as needed
        points.push(pt2(x, y));
    }
    return points;
}

