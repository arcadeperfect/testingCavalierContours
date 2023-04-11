use nannou::draw::properties::color;
use nannou::prelude::*;

use cavalier_contours::pline_open;
use cavalier_contours::polyline::*;

use rand::Rng;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {

    
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLACK);

    draw.ellipse().color(STEELBLUE);

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(100.0, 100.0))
        .color(RED);

    
    
    
    // array of points to form a line
    let pointss = [
        (pt2(0.0, 0.0), RED),
        (pt2(100.0, 100.0), GREEN),
        (pt2(200.0, 100.0), BLUE),
        (pt2(300.0, 0.0), YELLOW),
    ];
    
    let pointsss = [
        (pt2(0.0, 0.0)),
        (pt2(100.0, 100.0)),
        (pt2(200.0, 100.0)),
        (pt2(300.0, 0.0)),
    ];


    let mut rng = rand::thread_rng();
    let vertex_list: Vec<(f64, f64, f64)> = (0..10) // Change this number to the desired number of vertices
        .map(|_| {
            let x: f64 = rng.gen_range(-10.0..10.0); // Adjust the range as needed
            let y: f64 = rng.gen_range(-10.0..10.0); // Adjust the range as needed
            let bulge = 0.0;
            (x, y, bulge)
        })
        .collect();

    // Create a new polyline with the specified capacity and is_closed set to false
    let mut polyline = Polyline::with_capacity(vertex_list.len(), false);


    // Add the vertices from the generated list
    for (x, y, bulge) in vertex_list {
        polyline.add_vertex(PlineVertex::new(x, y, bulge));
    }


    let offset_plines = &polyline.parallel_offset(60.0)[0];

    
    let v = &offset_plines.vertex_data;

    let mut offsettedPoints: Vec<Vec2> = Vec::new();

    for i in 0..v.len() {
        let x = v[i].x as f32;
        let y = v[i].y as f32;
        let bulge = v[i].bulge;
        // println!("x: {}, y: {}, bulge: {}", x, y, bulge);
        // add to points
        offsettedPoints.push(pt2(x, y));
    }
    


    // draw line from points
    draw.polyline()
        .weight(3.0)
        .points(pointsss)
        .color(RED);
        // .points_colored(pointss);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}