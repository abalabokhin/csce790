use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct State {
    i: i32,
    j: i32,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Action {
    di: i32,
    dj: i32
}

fn do_action(st : &State, robot_action : &Action, nature_action : &Action, available_states : &Vec<State>) -> State {
    let new_state = State{i: st.i + robot_action.di + nature_action.di, j : st.j + robot_action.dj + nature_action.dj};
    match available_states.iter().position(|ref x| x.i == new_state.i && x.j == new_state.j) {
        Some(_x)    => new_state,
        None        => State{i: st.i, j: st.j}
    }
}

fn recalculate_cost_to_goals(
    available_states : &Vec<State>, 
    robot_actions : &Vec<Action>, 
    nature_behabiour : &HashMap<Action, f32>, 
    costs_to_goal : &Vec<f32>)->Vec<f32> {
    
    return vec![];
}

fn main() {
    let w = 15;
    let mut states = vec![]; 
    // Creating all states
    for x in 1..w + 1 {
        for y in 1..w + 1 {
            if x <= w / 3 || x > 2 * w / 3 || y <= w / 3 || y > 2 * w / 3 {
                states.push(State {i: x, j : y}); 
            }
        }
    }

    let infinity = 10000.;

    let costs_to_goal = states.iter().map(|ref x| if x.i > 2 * w / 3 && x.j > 2 * w / 3 {0.} else {infinity}).collect::<Vec<f32>>();

    let robot_actions = vec![Action{di : 0, dj : 0}, Action{di : 1, dj : 0}, Action{di : 0, dj : 1}, Action{di : -1, dj : 0}, Action{di : 0, dj : -1}];
    let mut no_nature_behaviour = HashMap::new();
    no_nature_behaviour.insert(Action{di : 0, dj : 0}, 1.);

    

    println!("{:?}", states);
    println!("{:?}", no_nature_behaviour);
    println!("{:?}", costs_to_goal);
}
