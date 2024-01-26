
use crate::{aabb::AABB, hittable2::Primitive, ray::Ray, hittable::Record};

#[derive(Debug)]
pub struct BVHNode {
    bounds: AABB,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    start: usize,
    end: usize,
}

const MAX_DEPTH: i32 = 2048;

impl BVHNode {
    pub fn default() -> Self {
        BVHNode { bounds: AABB::default(), left: None, right: None, start: 0, end: 0 }
    }

    pub fn new(&self, primitives: &Vec<Primitive>, indices: &mut Vec<usize>, start: usize, end: usize, depth: i32) -> Self {
        let extent = self.bounds.max - self.bounds.min;
        let mut axis = 0;
        if extent.v[0] < extent.v[1] {
            axis = 1;
        }
        if extent.v[axis] < extent.v[2] {
            axis = 2;
        }
        let num_obj = end - start;

        match num_obj {
            1 => {
                let bounds = (&primitives[start]).bounds().clone();
                let n =  BVHNode {
                    bounds,
                    left: None,
                    right: None,
                    start,
                    end,
                };
                return n;
            }, 
            _ => {
                if depth == MAX_DEPTH {
                    let mut bounds = AABB::default();
                    for i in start..end {
                        bounds.join(&(primitives[i]).bounds().clone());
                    }
                    return BVHNode {
                        bounds,
                        left: None,
                        right: None,
                        start,
                        end
                    }
                }

                let slice = &mut indices[start..end];
                slice.sort_by(|a,b| {
                    (&primitives[*a].centroid().v[axis]).partial_cmp(&primitives[*b].centroid().v[axis]).unwrap()
                });
                let mid = (start + end) / 2;
                let left = BVHNode::new(&self, primitives, indices, start, mid, depth + 1);
                let right = BVHNode::new(&self, primitives, indices, mid, end, depth + 1);

                let left_bounds = &left.bounds.clone();
                let right_bounds = &right.bounds.clone();
        
                let mut bounds = AABB::default();
                bounds.join(left_bounds);
                bounds.join(right_bounds);
        
                let n =  BVHNode {
                    bounds,
                    left: Some(Box::new(left)) ,
                    right: Some(Box::new(right)),
                    start,
                    end,
                };
                return n;
        
            }
        };
    }
    
    pub fn ray_hit(&self, primitives: &Vec<Primitive>, ray: &Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::Record> {
        match self {
            BVHNode { bounds, left:None , right: None, start, end } => {
                if !bounds.hit(ray) {
                    return None
                } else {
                    let mut final_record: Option<Record> = None;
                    for i in *start..*end {
                        let curr = primitives[i].ray_hit(ray, t_min, t_max);
                        match final_record {
                            Some(record) => {
                                match curr {
                                    Some(c) => if c.t < record.t {final_record = Some(c)},
                                    None => (),
                                }
                            },
                            None => {final_record = curr},
                        }
                        
                    }
                    return final_record;
                }
            },
            BVHNode { bounds, left, right, start: _, end: _ } => {
                if !bounds.hit(ray) {
                    return None
                } else {
                    let l = left.as_ref().unwrap().ray_hit(primitives, ray, t_min, t_max);
                    let r = right.as_ref().unwrap().ray_hit(primitives, ray, t_min, t_max);

                    return match l {
                        Some(l) =>  {match r {
                            Some(r) => if l.t < r.t {Some(l)} else {Some(r)},
                            None => Some(l)
                        }},
                        None => {
                            match r {
                                Some(r) => Some(r),
                                None => None
                            }
                        }
                    }
                }

            }
        }
    }
}


