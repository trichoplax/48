use std::f64::consts::TAU;
use svg::node::element::path::Data;
use svg::node::element::Circle;
use svg::node::element::Definitions;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::node::element::Use;
use svg::Document;

const ROOT_3: f64 = 1.7320508075688772;

fn main() {
    let board_size = 4; // Size 0 is a single hexagon, size N+1 is size N plus a ring of hexagons
    let hexagon_radius = 10.0;
    let hexagon_height = hexagon_radius * ROOT_3;
    let stroke_width = 1.0;
    let image_width =
        hexagon_radius * (5.0 + 3.0 * board_size as f64) + stroke_width * 2.0 / ROOT_3;
    let image_height = hexagon_height * (3.0 + 2.0 * board_size as f64) + stroke_width;
    let centre = Coordinates {
        x: image_width / 2.0,
        y: image_height / 2.0,
    };
    let dice_width = hexagon_radius;
    let hexagon_angle = TAU / 6.0;
    let hexagon_angle_in_degrees = hexagon_angle * 360.0 / TAU;
    let triangle_radius = hexagon_radius / 2.0;
    let triangle_angle = TAU / 3.0;
    let triangle_rotation = TAU / 4.0;
    let pip_radius = stroke_width;
    let board_cell_location_rotation = TAU / 12.0;
    let direction_hexagon_location_radius = hexagon_height * (board_size as f64 + 1.0);
    let direction_hexagon_location_rotation = 5.0 * hexagon_angle + board_cell_location_rotation;

    let hexagon_vertices: Vec<(f64, f64)> = (0..=5)
        .map(|n| {
            (
                centre.x + hexagon_radius * (n as f64 * hexagon_angle).cos(),
                centre.y + hexagon_radius * (n as f64 * hexagon_angle).sin(),
            )
        })
        .collect();

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

    let hexagon_definition = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", hexagon_data)
        .set("id", "hexagon");

    let triangle_definition = Path::new()
        .set("fill", "black")
        .set("stroke", "none")
        .set("d", triangle_data)
        .set("id", "triangle");

    let board_cell_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .add(Use::new().set("href", "#triangle"))
        .set("id", "board_cell");

    let dice_outline_definition = Rectangle::new()
        .set("x", centre.x - dice_width / 2.0)
        .set("y", centre.y - dice_width / 2.0)
        .set("width", dice_width)
        .set("height", dice_width)
        .set("stroke", "black")
        .set("fill", "none")
        .set("stroke-width", stroke_width)
        .set("rx", stroke_width)
        .set("id", "dice_outline");

    let pip_definition = Circle::new()
        .set("r", pip_radius)
        .set("cx", centre.x)
        .set("cy", centre.y)
        .set("fill", "black")
        .set("stroke", "none")
        .set("id", "pip");

    let centre_pip_definition = Use::new().set("href", "#pip").set("id", "centre_pip");

    let top_left_pip_definition = Use::new()
        .set("href", "#pip")
        .set("x", -dice_width / 4.0)
        .set("y", -dice_width / 4.0)
        .set("id", "top_left_pip");

    let top_right_pip_definition = Use::new()
        .set("href", "#pip")
        .set("x", dice_width / 4.0)
        .set("y", -dice_width / 4.0)
        .set("id", "top_right_pip");

    let bottom_left_pip_definition = Use::new()
        .set("href", "#pip")
        .set("x", -dice_width / 4.0)
        .set("y", dice_width / 4.0)
        .set("id", "bottom_left_pip");

    let bottom_right_pip_definition = Use::new()
        .set("href", "#pip")
        .set("x", dice_width / 4.0)
        .set("y", dice_width / 4.0)
        .set("id", "bottom_right_pip");

    let centre_left_pip_definition = Use::new()
        .set("href", "#pip")
        .set("x", -dice_width / 4.0)
        .set("id", "centre_left_pip");

    let centre_right_pip_definition = Use::new()
        .set("href", "#pip")
        .set("x", dice_width / 4.0)
        .set("id", "centre_right_pip");

    let dice_with_pips_1_definition = Group::new()
        .add(Use::new().set("href", "#dice_outline"))
        .add(Use::new().set("href", "#centre_pip"))
        .set("id", "dice_with_pips_1");

    let dice_with_pips_2_definition = Group::new()
        .add(Use::new().set("href", "#dice_outline"))
        .add(Use::new().set("href", "#top_left_pip"))
        .add(Use::new().set("href", "#bottom_right_pip"))
        .set("id", "dice_with_pips_2");

    let dice_with_pips_3_definition = Group::new()
        .add(Use::new().set("href", "#dice_outline"))
        .add(Use::new().set("href", "#top_left_pip"))
        .add(Use::new().set("href", "#centre_pip"))
        .add(Use::new().set("href", "#bottom_right_pip"))
        .set("id", "dice_with_pips_3");

    let dice_with_pips_4_definition = Group::new()
        .add(Use::new().set("href", "#dice_outline"))
        .add(Use::new().set("href", "#top_left_pip"))
        .add(Use::new().set("href", "#top_right_pip"))
        .add(Use::new().set("href", "#bottom_left_pip"))
        .add(Use::new().set("href", "#bottom_right_pip"))
        .set("id", "dice_with_pips_4");

    let dice_with_pips_5_definition = Group::new()
        .add(Use::new().set("href", "#dice_outline"))
        .add(Use::new().set("href", "#centre_pip"))
        .add(Use::new().set("href", "#top_left_pip"))
        .add(Use::new().set("href", "#top_right_pip"))
        .add(Use::new().set("href", "#bottom_left_pip"))
        .add(Use::new().set("href", "#bottom_right_pip"))
        .set("id", "dice_with_pips_5");

    let dice_with_pips_6_definition = Group::new()
        .add(Use::new().set("href", "#dice_outline"))
        .add(Use::new().set("href", "#top_left_pip"))
        .add(Use::new().set("href", "#top_right_pip"))
        .add(Use::new().set("href", "#bottom_left_pip"))
        .add(Use::new().set("href", "#bottom_right_pip"))
        .add(Use::new().set("href", "#centre_left_pip"))
        .add(Use::new().set("href", "#centre_right_pip"))
        .set("id", "dice_with_pips_6");

    let direction_hexagon_1_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .add(Use::new().set("href", "#dice_with_pips_1").set(
            "transform",
            format!(
                "rotate({},{},{})",
                hexagon_angle_in_degrees * 1.0,
                centre.x,
                centre.y
            ),
        ))
        .set("id", "direction_hexagon_1");

    let direction_hexagon_2_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .add(Use::new().set("href", "#dice_with_pips_2").set(
            "transform",
            format!(
                "rotate({},{},{})",
                hexagon_angle_in_degrees * 2.0,
                centre.x,
                centre.y
            ),
        ))
        .set("id", "direction_hexagon_2");

    let direction_hexagon_3_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .add(Use::new().set("href", "#dice_with_pips_3").set(
            "transform",
            format!(
                "rotate({},{},{})",
                hexagon_angle_in_degrees * 3.0,
                centre.x,
                centre.y
            ),
        ))
        .set("id", "direction_hexagon_3");

    let direction_hexagon_4_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .add(Use::new().set("href", "#dice_with_pips_4").set(
            "transform",
            format!(
                "rotate({},{},{})",
                hexagon_angle_in_degrees * 4.0,
                centre.x,
                centre.y
            ),
        ))
        .set("id", "direction_hexagon_4");

    let direction_hexagon_5_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .add(Use::new().set("href", "#dice_with_pips_5").set(
            "transform",
            format!(
                "rotate({},{},{})",
                hexagon_angle_in_degrees * 5.0,
                centre.x,
                centre.y
            ),
        ))
        .set("id", "direction_hexagon_5");

    let direction_hexagon_6_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .add(Use::new().set("href", "#dice_with_pips_6").set(
            "transform",
            format!(
                "rotate({},{},{})",
                hexagon_angle_in_degrees * 6.0,
                centre.x,
                centre.y
            ),
        ))
        .set("id", "direction_hexagon_6");

    let definitions = Definitions::new()
        .add(hexagon_definition)
        .add(triangle_definition)
        .add(board_cell_definition)
        .add(dice_outline_definition)
        .add(pip_definition)
        .add(centre_pip_definition)
        .add(top_left_pip_definition)
        .add(top_right_pip_definition)
        .add(bottom_left_pip_definition)
        .add(bottom_right_pip_definition)
        .add(centre_left_pip_definition)
        .add(centre_right_pip_definition)
        .add(dice_with_pips_1_definition)
        .add(dice_with_pips_2_definition)
        .add(dice_with_pips_3_definition)
        .add(dice_with_pips_4_definition)
        .add(dice_with_pips_5_definition)
        .add(dice_with_pips_6_definition)
        .add(direction_hexagon_1_definition)
        .add(direction_hexagon_2_definition)
        .add(direction_hexagon_3_definition)
        .add(direction_hexagon_4_definition)
        .add(direction_hexagon_5_definition)
        .add(direction_hexagon_6_definition);

    let direction_hexagon_coordinates: Vec<Coordinates> = (0..=5)
        .map(|n| Coordinates {
            x: direction_hexagon_location_radius
                * (n as f64 * hexagon_angle + direction_hexagon_location_rotation).cos(),
            y: direction_hexagon_location_radius
                * (n as f64 * hexagon_angle + direction_hexagon_location_rotation).sin(),
        })
        .collect();

    let direction_hexagon_1 = Use::new()
        .set("href", "#direction_hexagon_1")
        .set("x", direction_hexagon_coordinates[0].x)
        .set("y", direction_hexagon_coordinates[0].y);
    let direction_hexagon_2 = Use::new()
        .set("href", "#direction_hexagon_2")
        .set("x", direction_hexagon_coordinates[1].x)
        .set("y", direction_hexagon_coordinates[1].y);
    let direction_hexagon_3 = Use::new()
        .set("href", "#direction_hexagon_3")
        .set("x", direction_hexagon_coordinates[2].x)
        .set("y", direction_hexagon_coordinates[2].y);
    let direction_hexagon_4 = Use::new()
        .set("href", "#direction_hexagon_4")
        .set("x", direction_hexagon_coordinates[3].x)
        .set("y", direction_hexagon_coordinates[3].y);
    let direction_hexagon_5 = Use::new()
        .set("href", "#direction_hexagon_5")
        .set("x", direction_hexagon_coordinates[4].x)
        .set("y", direction_hexagon_coordinates[4].y);
    let direction_hexagon_6 = Use::new()
        .set("href", "#direction_hexagon_6")
        .set("x", direction_hexagon_coordinates[5].x)
        .set("y", direction_hexagon_coordinates[5].y);

    let mut document = Document::new()
        .set("viewBox", (0, 0, image_width, image_height))
        .add(definitions)
        .add(
            Rectangle::new()
                .set("width", image_width)
                .set("height", image_height)
                .set("fill", "none"),
        )
        .add(direction_hexagon_1)
        .add(direction_hexagon_2)
        .add(direction_hexagon_3)
        .add(direction_hexagon_4)
        .add(direction_hexagon_5)
        .add(direction_hexagon_6);

    for ring in 0..=board_size {
        if ring == 0 {
            document = document.add(Use::new().set("href", "#board_cell"));
        } else {
            for spoke in 0..=5 {
                for offset in 0..ring {
                    let spoke_angle = spoke as f64 * hexagon_angle + board_cell_location_rotation;
                    let ring_radius = ring as f64 * hexagon_height;
                    let offset_angle = spoke_angle + 2.0 * hexagon_angle;
                    let offset_distance = offset as f64 * hexagon_height;
                    document = document.add(
                        Use::new()
                            .set("href", "#board_cell")
                            .set(
                                "x",
                                ring_radius * (spoke_angle).cos()
                                    + offset_distance * offset_angle.cos(),
                            )
                            .set(
                                "y",
                                ring_radius * (spoke_angle).sin()
                                    + offset_distance * offset_angle.sin(),
                            ),
                    );
                }
            }
        }
    }

    svg::save("docs/board.svg", &document).unwrap();
}

struct Coordinates {
    x: f64,
    y: f64,
}
