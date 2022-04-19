use crate::space::line::get_new_line_segment;
use crate::space::line::Line;
use crate::space::node::Node;
use crate::space::node::TreeItem;
use crate::space::subsector::Subsector;
use crate::space::vec2d::Vec2d;

pub fn get_tree() -> Node {
    let normal_vector_simple_north = Vec2d::new(0.0, -1.0);
    let normal_vector_simple_south = Vec2d::new(0.0, 1.0);
    let normal_vector_simple_west = Vec2d::new(-1.0, 0.0);
    let normal_vector_simple_east = Vec2d::new(1.0, 0.0);

    let point_1_a = Vec2d::new(10.0, 10.0);
    let point_1_b = Vec2d::new(200.0, 10.0);
    let point_1_c = Vec2d::new(200.0, 100.0);
    let point_1_d_portal = Vec2d::new(150.0, 200.0); // P !!!
    let point_1_e_portal = Vec2d::new(100.0, 220.0); // P !!!
    let point_1_f = Vec2d::new(10.0, 200.0);

    let point_2_a = Vec2d::new(300.0, 300.0);
    // P //P
    let point_2_b = Vec2d::new(10.0, 300.0);
    let point_2_c = Vec2d::new(50.0, 500.0);
    let point_2_d = Vec2d::new(250.0, 600.0);
    let point_2_e = Vec2d::new(300.0, 450.0);

    // north polygon
    let line_1_a = get_new_line_segment(&point_1_a, &point_1_b, &normal_vector_simple_south); // 1st  4-5
    let line_1_b = get_new_line_segment(&point_1_b, &point_1_c, &normal_vector_simple_west); // 2nd  4-8
    let line_1_c = get_new_line_segment(&point_1_c, &point_1_d_portal, &normal_vector_simple_north); // 3rd  0-8
    let line_1_d = get_new_line_segment(
        &point_1_d_portal,
        &point_1_e_portal,
        &normal_vector_simple_north,
    ); // 4th  0-11 PORTAL
    let line_1_e = get_new_line_segment(&point_1_e_portal, &point_1_f, &normal_vector_simple_east); // 4th  0-11
    let line_1_f = get_new_line_segment(&point_1_f, &point_1_a, &normal_vector_simple_east); // 4th  0-11

    // south polygon
    let line_2_a = get_new_line_segment(&point_2_a, &point_1_d_portal, &normal_vector_simple_east); // 4th  0-11
                                                                                                    // line_2_b = Line(point_1_d_portal, point_1_e_portal, normal_vector_simple_east);  // 4th  0-11
                                                                                                    // line_1_d !!!!
    let line_2_c = get_new_line_segment(&point_1_e_portal, &point_2_b, &normal_vector_simple_east); // 4th  0-11
    let line_2_d = get_new_line_segment(&point_2_b, &point_2_c, &normal_vector_simple_east); // 4th  0-11
    let line_2_e = get_new_line_segment(&point_2_c, &point_2_d, &normal_vector_simple_east); // 4th  0-11
    let line_2_f = get_new_line_segment(&point_2_d, &point_2_e, &normal_vector_simple_east); // 4th  0-11
    let line_2_g = get_new_line_segment(&point_2_e, &point_2_a, &normal_vector_simple_east); // 4th  0-11

    let subsector_1 = Subsector::new(&vec![
        line_1_a.clone(),
        line_1_b.clone(),
        line_1_c.clone(),
        line_1_d.clone(),
        line_1_e.clone(),
        line_1_f.clone(),
        line_1_a.clone(),
    ]);
    let subsector_2 = Subsector::new(&vec![
        line_2_a.clone(),
        line_1_d.clone(),
        line_2_c.clone(),
        line_2_d.clone(),
        line_2_e.clone(),
        line_2_f.clone(),
        line_2_g.clone(),
    ]);
    let node = Node::new(
        line_1_d.clone(),
        None,
        None,
        Some(Box::new(subsector_1)),
        Some(Box::new(subsector_2)),
    );
    node
}
