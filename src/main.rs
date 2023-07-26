use std::f64::consts::TAU;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path as FilePath;
use svg::node::element::path::Data;
use svg::node::element::Circle;
use svg::node::element::Definitions;
use svg::node::element::Group;
use svg::node::element::Path as SVGPath;
use svg::node::element::Rectangle;
use svg::node::element::Use;
use svg::Document;

const ROOT_3: f64 = 1.7320508075688772;

fn main() {
    let board_size: u32 = 6; // Size 0 is a single hexagon, size N+1 is size N plus a ring of hexagons
    let number_of_hexagons = (board_size + 1).pow(3) - board_size.pow(3);
    let initial_dice_per_player = 12;
    let triangles_included = true;
    let triangles_hollow = true;
    let maximum_number_of_players = if triangles_included { 6 } else { 5 };
    let total_dice = number_of_hexagons + initial_dice_per_player * maximum_number_of_players;
    let hexagon_radius = 10.0;
    let hexagon_height = hexagon_radius * ROOT_3;
    let stroke_width = 1.0;
    let hexagon_rounded_corner_radius = stroke_width * 1.5;
    let dice_rounded_corner_radius = stroke_width * 1.0;
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
    let triangle_hollow_interior_radius = triangle_radius * 0.97;
    let triangle_angle = TAU / 3.0;
    let triangle_rotation = TAU / 4.0;
    let pip_radius = stroke_width;
    let board_cell_location_rotation = TAU / 12.0;
    let direction_hexagon_location_radius = hexagon_height * (board_size as f64 + 1.0);
    let direction_hexagon_location_rotation = 5.0 * hexagon_angle + board_cell_location_rotation;

    let hexagon_straight_line_endpoints: Vec<(f64, f64)> = (0..=5)
        .map(|n| {
            (
                centre.x
                    + hexagon_radius * (n as f64 * hexagon_angle).cos()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 4.0) * hexagon_angle).cos(),
                centre.y
                    + hexagon_radius * (n as f64 * hexagon_angle).sin()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 4.0) * hexagon_angle).sin(),
            )
        })
        .collect();

    let hexagon_rounded_corner_endpoints: Vec<(f64, f64)> = (0..=5)
        .map(|n| {
            (
                centre.x
                    + hexagon_radius * (n as f64 * hexagon_angle).cos()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 2.0) * hexagon_angle).cos(),
                centre.y
                    + hexagon_radius * (n as f64 * hexagon_angle).sin()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 2.0) * hexagon_angle).sin(),
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

    let triangle_hollow_interior_vertices: Vec<(f64, f64)> = (0..=2)
        .map(|n| {
            (
                centre.x
                    + triangle_hollow_interior_radius
                        * (n as f64 * triangle_angle + triangle_rotation).cos(),
                centre.y
                    + triangle_hollow_interior_radius
                        * (n as f64 * triangle_angle + triangle_rotation).sin(),
            )
        })
        .collect();

    let hexagon_data = Data::new()
        .move_to(hexagon_straight_line_endpoints[0])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[0].0,
            hexagon_rounded_corner_endpoints[0].1,
        ))
        .line_to(hexagon_straight_line_endpoints[1])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[1].0,
            hexagon_rounded_corner_endpoints[1].1,
        ))
        .line_to(hexagon_straight_line_endpoints[2])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[2].0,
            hexagon_rounded_corner_endpoints[2].1,
        ))
        .line_to(hexagon_straight_line_endpoints[3])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[3].0,
            hexagon_rounded_corner_endpoints[3].1,
        ))
        .line_to(hexagon_straight_line_endpoints[4])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[4].0,
            hexagon_rounded_corner_endpoints[4].1,
        ))
        .line_to(hexagon_straight_line_endpoints[5])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[5].0,
            hexagon_rounded_corner_endpoints[5].1,
        ))
        .close();

    let triangle_data = Data::new()
        .move_to(triangle_vertices[0])
        .line_to(triangle_vertices[1])
        .line_to(triangle_vertices[2])
        .close();

    let triangle_hollow_interior_data = Data::new()
        .move_to(triangle_hollow_interior_vertices[0])
        .line_to(triangle_hollow_interior_vertices[1])
        .line_to(triangle_hollow_interior_vertices[2])
        .close();

    let hexagon_definition = SVGPath::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", hexagon_data)
        .set("id", "hexagon");

    let triangle_definition = SVGPath::new()
        .set("fill", "black")
        .set("stroke", "none")
        .set("d", triangle_data)
        .set("id", "triangle");

    let triangle_hollow_interior_definition = SVGPath::new()
        .set("fill", "white")
        .set("stroke", "none")
        .set("d", triangle_hollow_interior_data)
        .set("id", "triangle_hollow_interior");

    let mut board_cell_definition = Group::new()
        .add(Use::new().set("href", "#hexagon"))
        .set("id", "board_cell");

    if triangles_included {
        board_cell_definition = board_cell_definition.add(Use::new().set("href", "#triangle"));
        if triangles_hollow {
            board_cell_definition =
                board_cell_definition.add(Use::new().set("href", "#triangle_hollow_interior"));
        }
    }

    let dice_outline_definition = Rectangle::new()
        .set("x", centre.x - dice_width / 2.0)
        .set("y", centre.y - dice_width / 2.0)
        .set("width", dice_width)
        .set("height", dice_width)
        .set("stroke", "black")
        .set("fill", "none")
        .set("stroke-width", stroke_width)
        .set("rx", dice_rounded_corner_radius)
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
        .add(triangle_hollow_interior_definition)
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

    // Paper calculations in centimetres
    let paper_height = 29.7; // Assuming portrait A4 - will be automatically
    let paper_width = 21.0; // switched to landscape later if necessary.
    let paper_margin = 0.6;
    let paper_image_overlap = 4.0;
    let paper_available_width = paper_width - 2.0 * paper_margin;
    let paper_available_height = paper_height - 2.0 * paper_margin;
    let paper_image_total_width = image_width * 3.0 / hexagon_height;
    let paper_image_total_height = image_height * 3.0 / hexagon_height;

    let portrait_sheets_required_for_width = ((paper_image_total_width - paper_image_overlap)
        / (paper_available_width - paper_image_overlap))
        .ceil() as u64;
    let portrait_sheets_required_for_height = ((paper_image_total_height - paper_image_overlap)
        / (paper_available_height - paper_image_overlap))
        .ceil() as u64;
    let landscape_sheets_required_for_width = ((paper_image_total_width - paper_image_overlap)
        / (paper_available_height - paper_image_overlap))
        .ceil() as u64;
    let landscape_sheets_required_for_height = ((paper_image_total_height - paper_image_overlap)
        / (paper_available_width - paper_image_overlap))
        .ceil() as u64;
    let portrait_sheets_required =
        portrait_sheets_required_for_width * portrait_sheets_required_for_height;
    let landscape_sheets_required =
        landscape_sheets_required_for_width * landscape_sheets_required_for_height;

    let paper_orientation = if portrait_sheets_required <= landscape_sheets_required {
        PaperOrientation::Portrait
    } else {
        PaperOrientation::Landscape
    };

    let (
        paper_oriented_available_width,
        paper_oriented_available_height,
        sheets_required_for_width,
        sheets_required_for_height,
    ) = match paper_orientation {
        PaperOrientation::Portrait => (
            paper_available_width,
            paper_available_height,
            portrait_sheets_required_for_width,
            portrait_sheets_required_for_height,
        ),
        PaperOrientation::Landscape => (
            paper_available_height,
            paper_available_width,
            landscape_sheets_required_for_width,
            landscape_sheets_required_for_height,
        ),
    };

    let (view_box_width, view_box_height, view_box_horizontal_step, view_box_vertical_step) = (
        image_width * paper_oriented_available_width / paper_image_total_width,
        image_height * paper_oriented_available_height / paper_image_total_height,
        image_width * (paper_oriented_available_width - paper_image_overlap)
            / paper_image_total_width,
        image_height * (paper_oriented_available_height - paper_image_overlap)
            / paper_image_total_height,
    );

    let views: Vec<(f64, f64, String)> = (0..sheets_required_for_height)
        .map(|y| {
            (0..sheets_required_for_width).map(move |x| {
                (
                    x as f64 * view_box_horizontal_step,
                    y as f64 * view_box_vertical_step,
                    format!("x{}y{}", x, y),
                )
            })
        })
        .flatten()
        .collect();

    let triangle_information = match (triangles_included, triangles_hollow) {
        (true, true) => "hollow_triangles",
        (true, false) => "filled_triangles",
        (_, _) => "no_triangles",
    };

    let board_description = format!(
        "size_{}_{}_board_with_{}",
        board_size, number_of_hexagons, triangle_information
    );

    let board_directory_name = format!("docs/{}", board_description);

    if !FilePath::new(&board_directory_name).exists() {
        let create_board_directory_result = fs::create_dir(&board_directory_name);

        match create_board_directory_result {
            Ok(_) => (),
            Err(error) => panic!(
                "Problem creating board directory {}:\n{:?}",
                board_directory_name, error
            ),
        };
    }

    let css_directory_name = format!("{}/css", board_directory_name);

    if !FilePath::new(&css_directory_name).exists() {
        let create_css_directory_result = fs::create_dir(&css_directory_name);

        match create_css_directory_result {
            Ok(_) => (),
            Err(error) => panic!(
                "Problem creating css directory {}:\n{:?}",
                css_directory_name, error
            ),
        };
    }

    document = document.set("viewBox", (0, 0, image_width, image_height));
    let svg_save_file_path = format!("{}/whole_board.svg", board_directory_name);
    let svg_save_result = svg::save(&svg_save_file_path, &document);

    match svg_save_result {
        Ok(_) => (),
        Err(error) => panic!(
            "Problem saving SVG file to {}:\n{:?}",
            svg_save_file_path, error
        ),
    };

    for (x, y, name) in &views {
        document = document.set("viewBox", (*x, *y, view_box_width, view_box_height));
        let svg_save_file_path = format!("{}/board_component_{}.svg", board_directory_name, name);
        let svg_save_result = svg::save(&svg_save_file_path, &document);

        match svg_save_result {
            Ok(_) => (),
            Err(error) => panic!(
                "Problem saving SVG file to {}:\n{:?}",
                svg_save_file_path, error
            ),
        };
    }

    let image_divs: String = views
        .iter()
        .map(|(_, _, name)| {
            format!(
                r#"<div><img src="board_component_{}.svg" alt="" /></div>"#,
                name
            )
        })
        .collect();

    let title_of_index_html = &board_description;
    let heading_of_index_html = &board_description;

    let content_of_index_html = format!(
        include_str!("templates/template_index_html.txt"),
        title_of_index_html, heading_of_index_html
    );

    let orientation_string = match paper_orientation {
        PaperOrientation::Portrait => "portrait",
        PaperOrientation::Landscape => "landscape",
    };

    let content_of_board_for_printing_html = format!(
        include_str!("templates/template_board_for_printing_html.txt"),
        orientation_string, image_divs
    );

    let content_of_dice_qr_code_svg = include_str!("templates/template_dice_qr_code_svg.txt");

    let content_of_rules_page_0_svg = include_str!("templates/template_rules_page_0_svg.txt");
    let content_of_rules_page_1_svg = format!(
        include_str!("templates/template_rules_page_1_svg.txt"),
        number_of_hexagons,
        total_dice,
        number_of_hexagons,
        initial_dice_per_player,
        maximum_number_of_players
    );
    let content_of_rules_page_2_svg = format!(
        include_str!("templates/template_rules_page_2_svg.txt"),
        initial_dice_per_player
    );
    let content_of_rules_page_3_svg = include_str!("templates/template_rules_page_3_svg.txt");
    let content_of_rules_page_4_svg = include_str!("templates/template_rules_page_4_svg.txt");
    let content_of_rules_page_5_svg = include_str!("templates/template_rules_page_5_svg.txt");
    let content_of_rules_page_6_svg = include_str!("templates/template_rules_page_6_svg.txt");
    let content_of_rules_page_7_svg = format!(
        include_str!("templates/template_rules_page_7_svg.txt"),
        content_of_dice_qr_code_svg
    );

    let content_of_rules_for_printing_html = format!(
        include_str!("templates/template_rules_for_printing_html.txt"),
        content_of_rules_page_4_svg,
        content_of_rules_page_3_svg,
        content_of_rules_page_2_svg,
        content_of_rules_page_1_svg,
        content_of_rules_page_5_svg,
        content_of_rules_page_6_svg,
        content_of_rules_page_7_svg,
        content_of_rules_page_0_svg
    );

    let content_of_board_css = format!(
        include_str!("templates/template_board_css.txt"),
        paper_oriented_available_height, paper_oriented_available_width
    );

    let content_of_board_print_css = format!(
        include_str!("templates/template_board_print_css.txt"),
        orientation_string
    );

    let content_of_rules_css = include_str!("templates/template_rules_css.txt");
    let content_of_rules_print_css = include_str!("templates/template_rules_print_css.txt");

    create_file(
        &format!("{}/index.html", board_directory_name),
        &content_of_index_html,
    );
    create_file(
        &format!("{}/board_for_printing.html", board_directory_name),
        &content_of_board_for_printing_html,
    );
    create_file(
        &format!("{}/rules_for_printing.html", board_directory_name),
        &content_of_rules_for_printing_html,
    );
    create_file(
        &format!("{}/board.css", css_directory_name),
        &content_of_board_css,
    );
    create_file(
        &format!("{}/board_print.css", css_directory_name),
        &content_of_board_print_css,
    );
    create_file(
        &format!("{}/rules.css", css_directory_name),
        &content_of_rules_css,
    );
    create_file(
        &format!("{}/rules_print.css", css_directory_name),
        &content_of_rules_print_css,
    );
}

struct Coordinates {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum PaperOrientation {
    Portrait,
    Landscape,
}

fn create_file(file_path_name: &str, file_content: &str) {
    let create_file_path_result = File::create(&file_path_name);

    let mut file_path = match create_file_path_result {
        Ok(file) => file,
        Err(error) => panic!(
            "Problem creating file path for {}:\n{:?}",
            file_path_name, error
        ),
    };

    let write_file_result = write!(file_path, "{}", file_content);

    match write_file_result {
        Ok(_) => (),
        Err(error) => panic!("Problem writing to {:?}:\n{:?}", file_path, error),
    };
}
