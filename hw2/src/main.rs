extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::cmp;
use std::f32;
use std::i32;

#[derive(Debug, Clone)]
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

    fn area(&self) -> f32 {
        (self.x0[1] - self.x0[0]) * (self.x1[1] - self.x1[0])
    }

    fn center(&self) -> (f32, f32) {
        ((self.x0[1] + self.x0[0]) / 2., (self.x1[1] + self.x1[0]) / 2.)
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

fn generate_nature_action_randomly()->Action {
    let between = Range::new(0f32, 1.);
    let mut rng = rand::thread_rng();
    let x0 = between.ind_sample(&mut rng) - 0.25;
    let x1 = between.ind_sample(&mut rng) - 0.75;
    return Action{ dx0 : [x0, x0], dx1 : [x1, x1] };
}

fn generate_robot_action_randomly()->Action {
    let between = Range::new(-5f32, 5.);
    let mut rng = rand::thread_rng();
    let x0 = between.ind_sample(&mut rng);
    let x1 = between.ind_sample(&mut rng);
    return Action{ dx0 : [x0, x0], dx1 : [x1, x1] };
}

fn generate_smart_robot_action(st : &State)->Action {
    let center = st.center();
    let mut x0 = -0.25 - center.0;
    let mut x1 = 0.25 - center.1;

    if x0 < -5. { x0 = -5. }
    if x0 > 5. { x0 = 5. }
    if x1 < -5. { x1 = -5. }
    if x1 > 5. { x1 = 5. }

    return Action { dx0 : [x0, x0], dx1 : [x1, x1] };
}

fn do_observation(st: &State)->Observation {
    let between = Range::new(-1.5f32, 1.5);
    let mut rng = rand::thread_rng();
    let x = between.ind_sample(&mut rng);

    let between1 = Range::new(0, 2);
    let axis = between1.ind_sample(&mut rng);

    match axis {
        0 => { return Observation::new(0, st.x0[0] + x); }
        _ => { return Observation::new(1, st.x1[0] + x); }
    }
}

fn main() {
    let nature_distortion = Action { dx0: [-0.25, 0.75], dx1: [-0.75, 0.25] };
    let mut st1 = State{ x0: [-10., 10.], x1: [-10., 10.] }; 
    println!("4.");
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

    println!("5.");

    let mut min_area = f32::MAX;
    let mut max_area = 0f32;
    let mut min_area_state = State{x0: [0., 0.], x1: [0., 0.,]};
    let mut max_area_state = State{x0: [0., 0.], x1: [0., 0.,]};

    let mut nondetermenistic_state = State{ x0: [-10., 10.], x1: [-10., 10.] };
    let mut determenistic_state = State{ x0: [0., 0.], x1: [0., 0.] };     
    let obs = do_observation(&determenistic_state);
    nondetermenistic_state.refine_with_observation(&obs);
  
    for _ in 0..500 {
        let area = nondetermenistic_state.area();
        println!("{} ", area);
        if area > max_area {
            max_area = area;
            max_area_state = nondetermenistic_state.clone();
        }
        if area < min_area {
            min_area = area;
            min_area_state = nondetermenistic_state.clone();
        }

        let nature_action = generate_nature_action_randomly();
        let robot_action = generate_robot_action_randomly();
        determenistic_state.forward_projection(&robot_action, &nature_action);
        nondetermenistic_state.forward_projection(&robot_action, &nature_distortion);
        let obs = do_observation(&determenistic_state);
        nondetermenistic_state.refine_with_observation(&obs);
    }

    println!("");
    println!("state {:?} has min area {}", min_area_state, min_area);
    println!("state {:?} has max area {}", max_area_state, max_area);

    println!("6.c");

    let mut total_steps_count = 0;
    let mut min_steps_count = i32::MAX;
    let mut max_steps_count = 0;
    
    for _ in 0..1000 {
        let mut nondetermenistic_state = State{ x0: [-10., 10.], x1: [-10., 10.] };
        let mut determenistic_state = State{ x0: [9.5, 9.5], x1: [9.5, 9.5] };     
        let obs = do_observation(&determenistic_state);
        nondetermenistic_state.refine_with_observation(&obs);

        let mut steps_count = 0;
        while !nondetermenistic_state.is_goal() {
            steps_count += 1;
            let robot_action = generate_smart_robot_action(&nondetermenistic_state);
            let nature_action = generate_nature_action_randomly();
            determenistic_state.forward_projection(&robot_action, &nature_action);
            nondetermenistic_state.forward_projection(&robot_action, &nature_distortion);
            let obs = do_observation(&determenistic_state);
            nondetermenistic_state.refine_with_observation(&obs);
        }
        total_steps_count += steps_count;
        min_steps_count = cmp::min(min_steps_count, steps_count);
        max_steps_count = cmp::max(max_steps_count, steps_count);
    }
    println!("average steps amount to reach the goal: {}", total_steps_count as f32 / 1000.);
    println!("min steps to reach the goal: {}", min_steps_count);
    println!("max steps to reach the goal: {}", max_steps_count);
} 
