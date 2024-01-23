use std::{cell::RefCell, rc::Rc};

use crate::{aabb::AABB, hittable2::Primitive, ray::Ray, hittable::Hittable};

#[derive(Debug)]
pub struct BVHNode {
    bounds: AABB,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    start: usize,
    end: usize,
}


impl BVHNode {
    pub fn default() -> Self {
        BVHNode { bounds: AABB::default(), left: None, right: None, start: 0, end: 0 }
    }

    pub fn new(&self, primitives: &Vec<Primitive>, indices: &mut Vec<usize>, start: usize, end: usize) -> Self {
        // println!("{start} {end}");
        let extent = self.bounds.max - self.bounds.min;
        let mut axis = 0;
        if extent.v[0] < extent.v[1] {
            axis = 1;
        }
        if extent.v[axis] < extent.v[2] {
            axis = 2;
        }

        // let split = self.bounds.min.v[axis] + (extent.v[axis] * 0.5);
        let num_obj = end - start;
        // let left: BVHNode;
        // let right: BVHNode;

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
                // println!("{:?}", n);
                return n;
            }, 
            _ => {
                let slice = &mut indices[start..end];
                slice.sort_by(|a,b| {
                    (&primitives[*a].centroid().v[axis]).partial_cmp(&primitives[*b].centroid().v[axis]).unwrap()
                });
                let mid = (start + end) / 2;
                let left = BVHNode::new(&self, primitives, indices, start, mid);
                let right = BVHNode::new(&self, primitives, indices, mid, end);

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
                    return primitives[*start].ray_hit(ray, t_min, t_max)
                }
            },
            BVHNode { bounds, left, right, start, end } => {
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

// struct BVHTree {
//     max_node: usize,
//     max_depth: usize,
//     root: RefNode,
// }

// impl BVHTree {
//     pub fn build(world:&mut [Hittable_Enum], max_node: usize, max_depth: usize) -> Self {
//         let mut bvh = BVHTree { max_node, max_depth, root: None };
//         let size = world.len();
//         bvh.root = Some(RefCell::new(Rc::new(BVHNode::new(world, 0, 0, size))));
//         BVHTree { max_node, max_depth, root: None }
//     }
// }

// struct BVHNode {
//     bounds: AABB,
//     level: usize,
//     world: &mut Vec<Box<Hittable_Enum>>,
//     start: usize,
//     end: usize,
//     left: Option<RefCell<Rc<BVHNode>>>,
//     right: Option<RefCell<Rc<BVHNode>>>,
// }



// impl BVHNode {
//     pub fn new(world: &mut [Hittable_Enum], level: usize, start: usize, end: usize) -> BVHNode {
//         let mut bounds: AABB = AABB::default();
//         let mut node:BVHNode = BVHNode { bounds, level, world, left: None, right: None, start, end};
//         node.subdivide()
//     }

//     pub fn subdivide(mut self) -> Self {



        

//         while i < j {
            
//             // if self.list.borrow().borrow().get(i).unwrap().centroid().v[axis] < split {
//             // if self.world.list[i].centroid().v[axis] < split {
//             if self.world.borrow().list[i].centroid().v[axis] < split {

//             // if self.world.list[i].centroid()
//             // if self.world.list.get(i).unwrap().centroid().v[axis] < split {
//                 i += 1;
//             } else {
//                 self.world.borrow_mut().list.swap(i, j);
//             //     self.world.swap(i, j);
//                 j -= 1;
//             }
//         }

//         let l:BVHNode = BVHNode::new(self.world, self.level + 1, self.start, i);
//         let r:BVHNode = BVHNode::new(self.world, self.level + 1, i, self.end);

//         self.left = Some(RefCell::new(Rc::new(l)));

//         // let left: BVHNode = BVHNode::new(self.world, self.level + 1, self.start, i);
//         // let right = BVHNode::new(self.world, self.level + 1, i, self.end);

//         // self.left = Some(RefCell::new(Rc::new(left)));
//         // self.right = Some(RefCell::new(Rc::new(right)));
        
//         self
//     }
// }

pub type RefNode = Option<RefCell<Rc<BVHNode>>>;
