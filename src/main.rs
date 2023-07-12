use std::f64::consts::TAU;
use svg::node::element::path::Data;
use svg::node::element::Definitions;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::node::element::Use;
use svg::Document;

fn main() {
    let centre = Coordinates { x: 100.0, y: 100.0 };
    let hexagon_radius = 90.0;
    let hexagon_angle = TAU / 6.0;

    let hexagon_vertices: Vec<(f64, f64)> = (0..=5)
        .map(|n| {
            (
                centre.x + hexagon_radius * (n as f64 * hexagon_angle).cos(),
                centre.y + hexagon_radius * (n as f64 * hexagon_angle).sin(),
            )
        })
        .collect();

    let triangle_radius = 45.0;
    let triangle_angle = TAU / 3.0;
    let triangle_rotation = TAU / 4.0;

    let triangle_vertices: Vec<(f64, f64)> = (0..=2)
        .map(|n| {
            (
                centre.x + triangle_radius * (n as f64 * triangle_angle + triangle_rotation).cos(),
                centre.y + triangle_radius * (n as f64 * triangle_angle + triangle_rotation).sin(),
            )
        })
        .collect();

    let hexagon_data = Data::new()
        .move_to(hexagon_vertices[0])
        .line_to(hexagon_vertices[1])
        .line_to(hexagon_vertices[2])
        .line_to(hexagon_vertices[3])
        .line_to(hexagon_vertices[4])
        .line_to(hexagon_vertices[5])
        .close();

    let triangle_data = Data::new()
        .move_to(triangle_vertices[0])
        .line_to(triangle_vertices[1])
        .line_to(triangle_vertices[2])
        .close();

    let hexagon_path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 10)
        .set("d", hexagon_data)
        .set("id", "hexagon");

    let triangle_path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 10)
        .set("d", triangle_data)
        .set("id", "triangle");

    let use_hexagon = Use::new().set("href", "#hexagon");

    let use_triangle = Use::new().set("href", "#triangle");

    let board_cell_group = Group::new()
        .add(use_hexagon)
        .add(use_triangle)
        .set("id", "board_cell");

    let definitions = Definitions::new()
        .add(hexagon_path)
        .add(triangle_path)
        .add(board_cell_group);

    let use_board_cell = Use::new().set("href", "#board_cell");

    let document = Document::new()
        .set("viewBox", (0, 0, 200, 200))
        .add(definitions)
        .add(use_board_cell);

    svg::save("board.svg", &document).unwrap();
}

struct Coordinates {
    x: f64,
    y: f64,
}
