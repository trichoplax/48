use std::f64::consts::TAU;
use svg::node::element::path::Data;
use svg::node::element::Definitions;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::node::element::Use;
use svg::Document;

const ROOT_3: f64 = 1.7320508075688772;

fn main() {
    let centre = Coordinates { x: 100.0, y: 100.0 };
    let stroke_width = 3;
    let hexagon_radius = 30.0;
    let hexagon_angle = TAU / 6.0;

    let hexagon_vertices: Vec<(f64, f64)> = (0..=5)
        .map(|n| {
            (
                centre.x + hexagon_radius * (n as f64 * hexagon_angle).cos(),
                centre.y + hexagon_radius * (n as f64 * hexagon_angle).sin(),
            )
        })
        .collect();

    let triangle_radius = hexagon_radius / 2.0;
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
        .set("stroke-width", stroke_width)
        .set("d", hexagon_data)
        .set("id", "hexagon");

    let triangle_path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", triangle_data)
        .set("id", "triangle");

    let hexagon = Use::new().set("href", "#hexagon");

    let triangle = Use::new().set("href", "#triangle");

    let board_cell_group = Group::new()
        .add(hexagon)
        .add(triangle)
        .set("id", "board_cell");

    let definitions = Definitions::new()
        .add(hexagon_path)
        .add(triangle_path)
        .add(board_cell_group);

    let board_cell = Use::new().set("href", "#board_cell");
    let board_cell_location_rotation = TAU / 12.0;
    let direction_hexagon_location_radius = hexagon_radius * ROOT_3;

    let direction_hexagon_coordinates: Vec<Coordinates> = (0..=5)
        .map(|n| Coordinates {
            x: centre.x
                + direction_hexagon_location_radius
                    * (n as f64 * hexagon_angle + board_cell_location_rotation).cos(),
            y: centre.y
                + direction_hexagon_location_radius
                    * (n as f64 * hexagon_angle + board_cell_location_rotation).sin(),
        })
        .collect();

    let direction_hexagon_1 = Use::new()
        .set("href", "#hexagon")
        .set("x", direction_hexagon_coordinates[0].x - centre.x)
        .set("y", direction_hexagon_coordinates[0].y - centre.y);
    let direction_hexagon_2 = Use::new()
        .set("href", "#hexagon")
        .set("x", direction_hexagon_coordinates[1].x - centre.x)
        .set("y", direction_hexagon_coordinates[1].y - centre.y);
    let direction_hexagon_3 = Use::new()
        .set("href", "#hexagon")
        .set("x", direction_hexagon_coordinates[2].x - centre.x)
        .set("y", direction_hexagon_coordinates[2].y - centre.y);
    let direction_hexagon_4 = Use::new()
        .set("href", "#hexagon")
        .set("x", direction_hexagon_coordinates[3].x - centre.x)
        .set("y", direction_hexagon_coordinates[3].y - centre.y);
    let direction_hexagon_5 = Use::new()
        .set("href", "#hexagon")
        .set("x", direction_hexagon_coordinates[4].x - centre.x)
        .set("y", direction_hexagon_coordinates[4].y - centre.y);
    let direction_hexagon_6 = Use::new()
        .set("href", "#hexagon")
        .set("x", direction_hexagon_coordinates[5].x - centre.x)
        .set("y", direction_hexagon_coordinates[5].y - centre.y);

    let document = Document::new()
        .set("viewBox", (0, 0, 200, 200))
        .add(definitions)
        .add(board_cell)
        .add(direction_hexagon_1)
        .add(direction_hexagon_2)
        .add(direction_hexagon_3)
        .add(direction_hexagon_4)
        .add(direction_hexagon_5)
        .add(direction_hexagon_6);

    svg::save("board.svg", &document).unwrap();
}

struct Coordinates {
    x: f64,
    y: f64,
}
