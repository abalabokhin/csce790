//extern crate rand;

//use std::collections::BTreeMap;
//use std::collections::HashMap;
//use rand::distributions::{IndependentSample, Range};
//use std::cmp;

#[derive(Debug)]
struct State {
    x0: [f32; 2],
    x1: [f32; 2],
}

impl State {
    fn is_goal(&self) -> bool {
        self.x0[0] >= -2. && self.x0[1] <= 2. && self.x1[0] >= -2. && self.x1[1] <= 2.
    }


    fn refine_with_observation(&mut self, ob: &Observation) {
        match ob.axis {
            0=> { self.x0[0] = self.x0[0].max(ob.x[0]); self.x0[1] = self.x0[1].min(ob.x[1]); }
            _=> { self.x1[0] = self.x1[0].max(ob.x[0]); self.x1[1] = self.x1[1].min(ob.x[1]); }
        }
    }

    fn forward_projection(&mut self, robot_action : &Action, nature_action : &Action) {
        self.x0[0] += robot_action.dx0[0] + nature_action.dx0[0];
        self.x0[1] += robot_action.dx0[1] + nature_action.dx0[1];
        self.x1[0] += robot_action.dx1[0] + nature_action.dx1[0];
        self.x1[1] += robot_action.dx1[1] + nature_action.dx1[1];
    }
}

#[derive(Debug)]
struct Action {
    dx0: [f32; 2],
    dx1: [f32; 2],
}

impl Action {
    fn new_det(x0: f32, x1: f32) -> Action {
        Action { dx0: [x0, x0], dx1: [x1, x1]}
    }
}

#[derive(Debug)]
struct Observation {
    axis: i32, 
    x: [f32; 2],
}

impl Observation {
    fn new(a: i32, value: f32) -> Observation {
        Observation { axis: a, x: [value - 1.5, value + 1.5]}
    }
}

fn main() {
    let nature_distortion = Action { dx0: [-0.25, 0.75], dx1: [-0.75, 0.25] };
    let mut st1 = State{ x0: [-10., 10.], x1: [-10., 10.] }; 
    println!("3.");
    println!("{:?}", st1); 
    let ob1 = Observation::new(0, 0.5);
    st1.refine_with_observation(&ob1);                               
    println!("{:?}, {:?}", ob1, st1);
    let act1 = Action::new_det ( 2., 2. );
    st1.forward_projection(&act1, &nature_distortion);
    println!("{:?}, {:?}", act1, st1);
    let ob2 = Observation::new(1, 6.);
    st1.refine_with_observation(&ob2);
    println!("{:?}, {:?}", ob2, st1);
    st1.forward_projection(&act1, &nature_distortion);
    println!("{:?}, {:?}", act1, st1);

    println!("4.");





} 
