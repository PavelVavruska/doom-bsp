use crate::space::line::get_new_line_segment;
use crate::space::node::Node;
use crate::space::subsector::Subsector;
use crate::space::vec2d::Vec2d;

pub fn get_tree() -> Node {
    /*
    Map:
    ###   ###
    #########
    ###   ###

    Segments:
    1st  2nd 4th   3rd
    ###            ###
    ### | # | ## | ###
    ###            ###

    */
    // simple normal vectors that can be found in squeres
    let normal_vector_simple_north = Vec2d::new(0.0, -1.0);
    let normal_vector_simple_south = Vec2d::new(0.0, 1.0);
    let normal_vector_simple_west = Vec2d::new(-1.0, 0.0);
    let normal_vector_simple_east = Vec2d::new(1.0, 0.0);

    // points of 1st subsectors in map clockwise
    // top
    let point_1_a = Vec2d::new(0.0, 0.0);
    let point_1_b = Vec2d::new(300.0, 0.0);
    // portal in the middle
    let point_1_c_portal = Vec2d::new(300.0, 100.0); // P portal point of first segment
    let point_1_d_portal = Vec2d::new(300.0, 200.0); // P portal point of first segment
                                                     // bottom
    let point_1_e = Vec2d::new(300.0, 300.0);
    let point_1_f = Vec2d::new(0.0, 300.0);

    // points of 2st subsectors in map clockwise
    // top
    let point_2_a = Vec2d::new(300.0, 100.0);
    let point_2_b = Vec2d::new(400.0, 100.0);
    // bottom
    let point_2_c = Vec2d::new(300.0, 200.0);
    let point_2_d = Vec2d::new(400.0, 200.0);
    // portal in the middle west - duplicated points for readibility
    let point_2_e_portal = Vec2d::new(300.0, 200.0);
    let point_2_f_portal = Vec2d::new(300.0, 100.0);

    // points of 3st subsectors in map clockwise
    // top
    let point_3_a = Vec2d::new(600.0, 0.0);
    let point_3_b = Vec2d::new(900.0, 0.0);
    // bottom
    let point_3_c = Vec2d::new(900.0, 300.0);
    let point_3_d = Vec2d::new(600.0, 300.0);
    // portal in the middle
    let point_3_e_portal = Vec2d::new(600.0, 100.0);
    let point_3_f_portal = Vec2d::new(600.0, 200.0);

    // points of 4st subsectors in map clockwise
    // top
    let point_4_a = Vec2d::new(400.0, 100.0);
    let point_4_b = Vec2d::new(600.0, 100.0);
    // portal in the middle
    let point_4_c_portal = Vec2d::new(600.0, 100.0);
    let point_4_d_portal = Vec2d::new(600.0, 200.0);
    // bottom
    let point_4_e = Vec2d::new(400.0, 200.0);
    let point_4_f = Vec2d::new(600.0, 200.0);

    // points of main cut
    let point_main_p_1 = Vec2d::new(400.0, 100.0);
    let point_main_p_2 = Vec2d::new(400.0, 200.0);

    // lines clockwise

    // segment 1 - big left
    let line_1_a = get_new_line_segment(&point_1_a, &point_1_b, &normal_vector_simple_south, false);
    let line_1_b = get_new_line_segment(
        &point_1_b,
        &point_1_c_portal,
        &normal_vector_simple_west,
        false,
    );
    let line_1_c_portal = get_new_line_segment(
        &point_1_c_portal,
        &point_1_d_portal,
        &normal_vector_simple_west,
        true,
    );
    let line_1_d = get_new_line_segment(
        &point_1_d_portal,
        &point_1_e,
        &normal_vector_simple_west,
        false,
    );
    let line_1_e = get_new_line_segment(&point_1_e, &point_1_f, &normal_vector_simple_north, false);
    let line_1_f = get_new_line_segment(&point_1_f, &point_1_a, &normal_vector_simple_east, false);

    // segment 2 small left
    let line_2_a = get_new_line_segment(&point_2_a, &point_2_b, &normal_vector_simple_south, false);
    let line_2_b = get_new_line_segment(&point_2_c, &point_2_d, &normal_vector_simple_north, false);
    let line_2_c_portal = get_new_line_segment(
        &point_2_e_portal,
        &point_2_f_portal,
        &normal_vector_simple_east,
        true,
    );
    let line_2_d = get_new_line_segment(
        &point_main_p_1,
        &point_main_p_2,
        &normal_vector_simple_west,
        true,
    );

    // segment 3 - big right
    let line_3_a = get_new_line_segment(&point_3_a, &point_3_b, &normal_vector_simple_south, false);
    let line_3_b = get_new_line_segment(&point_3_b, &point_3_c, &normal_vector_simple_west, false);
    let line_3_c = get_new_line_segment(&point_3_c, &point_3_d, &normal_vector_simple_north, false);
    let line_3_d = get_new_line_segment(
        &point_3_d,
        &point_3_f_portal,
        &normal_vector_simple_east,
        false,
    );
    let line_3_e_portal = get_new_line_segment(
        &point_3_f_portal,
        &point_3_e_portal,
        &normal_vector_simple_east,
        true,
    );
    let line_3_f = get_new_line_segment(
        &point_3_e_portal,
        &point_3_a,
        &normal_vector_simple_east,
        false,
    );

    // segment 4 - small right
    let line_4_a = get_new_line_segment(&point_4_a, &point_4_b, &normal_vector_simple_south, false);
    let line_4_b_portal = get_new_line_segment(
        &point_4_c_portal,
        &point_4_d_portal,
        &normal_vector_simple_west,
        true,
    );
    let line_4_c = get_new_line_segment(&point_4_f, &point_4_e, &normal_vector_simple_north, false);

    let line_main_portal_to_seg_1 = get_new_line_segment(
        &point_main_p_2,
        &point_main_p_1,
        &normal_vector_simple_west,
        true,
    );

    let line_main_portal_to_seg_3 = get_new_line_segment(
        &point_main_p_2,
        &point_main_p_1,
        &normal_vector_simple_east,
        true,
    );

    // subsectors

    let subsector_1 = Subsector::new(&vec![
        line_1_a.clone(),
        line_1_b.clone(),
        line_1_c_portal.clone(),
        line_1_d.clone(),
        line_1_e.clone(),
        line_1_f.clone(),
    ]);
    let subsector_2 = Subsector::new(&vec![
        line_2_a.clone(),
        line_2_b.clone(),
        line_2_c_portal.clone(),
        line_2_d.clone(),
    ]);
    let subsector_3 = Subsector::new(&vec![
        line_3_a.clone(),
        line_3_b.clone(),
        line_3_c.clone(),
        line_3_d.clone(),
        line_3_e_portal.clone(),
        line_3_f.clone(),
    ]);
    let subsector_4 = Subsector::new(&vec![
        line_4_a.clone(),
        line_4_b_portal.clone(),
        line_4_c.clone(),
        line_main_portal_to_seg_3.clone(),
    ]);

    let node_1_seg_1_seg2 = Node::new(
        line_1_c_portal.clone(),
        None,
        None,
        Some(Box::new(subsector_1)),
        Some(Box::new(subsector_2)),
    );

    let node_2_seg_3_seg4 = Node::new(
        line_3_e_portal.clone(),
        None,
        None,
        Some(Box::new(subsector_4)),
        Some(Box::new(subsector_3)),
    );
    let node_3_node12 = Node::new(
        line_main_portal_to_seg_1.clone(),
        Some(Box::new(node_1_seg_1_seg2)),
        Some(Box::new(node_2_seg_3_seg4)),
        None,
        None,
    );
    //node_1_seg_1_seg2 //
    node_3_node12
}
