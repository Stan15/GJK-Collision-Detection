use crate::vec2::Vec2;
use crate::shape::shape::ConvexShape;

struct GJK;
impl CollisionMethod for GJK {
    fn has_collided(s1: ConvexShape, s2: ConvexShape) {
        gjk(s1, s2);
    }
}

fn gjk(s1: impl Collidable, s2: impl Collidable) -> bool {
    let mut d = Vec2 { x: 1.0, y: 0.0 };

    let mut simplex = vec![support(&s1, &s2, d)];
    d = Vec2::origin() - simplex[0];

    let has_collided = loop {
        let new_support = support(&s1, &s2, d);
        // if new support point does not cross origin
        if new_support.dot(&d) < 0.0 {
            break false;
        }
        if simplex.len() == 3 {
            simplex.remove(0);
        }
        simplex.push(new_support);
        match new_direction(&mut simplex) {
            Some(new_d) => { d = new_d; }
            None => { break true; }
        }
    };
    has_collided
}

fn support(s1: &impl Collidable, s2: &impl Collidable, direction: Vec2) -> Vec2 {
    s1.support_point(direction) - s2.support_point(-direction)
}

fn new_direction(simplex: &mut Vec<Vec2>) -> Option<Vec2> {
    // collision exists
    if is_origin_on_edge(simplex) { return None }

    match simplex[..] {
        [B, A] => { Some(perp_line_toward_origin((B, A))) }
        [C, B, A] => { 
            let (AB, AC, AO) = (B-A, C-A, Vec2::origin() - A);
            let AB_perp = Vec2::tripple_prod(&AC, &AB, &AB);
            let AC_perp = Vec2::tripple_prod(&AB, &AC, &AC);
            
            if AB_perp.dot(&AO) > 0.0 {
                simplex.remove(0);
                return Some(AB_perp)
            } else if AC_perp.dot(&AO) > 0.0 {
                simplex.remove(1);
                return Some(AC_perp)
            } else {
                // origin is inside triangle. collision exists
                return None
            }
        }
        _ => { panic!("Invalid simpex size") }
    }
}

fn is_origin_on_edge(simplex: &Vec<Vec2>) -> bool {
    for i in 1..simplex.len() {
        let point_a = simplex[i];
        for j in (i+1)..simplex.len() {
            let point_b = simplex[j];
            if point_a.y*(point_b.x-point_a.x) 
                == point_a.x*(point_b.y-point_a.y) {
                return true
            }
        }
    }
    false
}

fn perp_line_toward_origin(simplex_1d: (Vec2, Vec2)) -> Vec2 {
    let (B, A) = simplex_1d;
    let (AB, AO) = (B - A, Vec2::origin() - A);
    Vec2::tripple_prod(&AB, &AO, &AB)
}
