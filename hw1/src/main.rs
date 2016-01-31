use std::collections::HashMap;

const INFINITY: f32 = 10000.;
const W: i32 = 15;

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

fn is_goal(x: &State) -> bool { x.i > 2 * W / 3 && x.j > 2 * W / 3 }

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
    
    let mut new_costs_to_goal = vec![];
    for ref state in available_states {
        let mut min_cost_to_goal = if is_goal(state) {0.} else {INFINITY}; 
        for ref action in robot_actions {
            let mut current_cost_to_goal = 0.;
            for ref nature_action in nature_behabiour {
            }
            if current_cost_to_goal < min_cost_to_goal {
                min_cost_to_goal = current_cost_to_goal;
            }
        }
        new_costs_to_goal.push(min_cost_to_goal);
    }
    return new_costs_to_goal;
}

fn main() {
    let mut states = vec![]; 
    // Creating all states
    for x in 1..W + 1 {
        for y in 1..W + 1 {
            if x <= W / 3 || x > 2 * W / 3 || y <= W / 3 || y > 2 * W / 3 {
                states.push(State {i: x, j : y}); 
            }
        }
    }

    let mut costs_to_goal = states.iter().map(|ref x| if is_goal(x) {0.} else {INFINITY}).collect::<Vec<f32>>();

    let robot_actions = vec![Action{di : 0, dj : 0}, Action{di : 1, dj : 0}, Action{di : 0, dj : 1}, Action{di : -1, dj : 0}, Action{di : 0, dj : -1}];
    let mut no_nature_behaviour = HashMap::new();
    no_nature_behaviour.insert(Action{di : 0, dj : 0}, 1.);

    for x in 1..1000 {
        costs_to_goal = recalculate_cost_to_goals(&states, &robot_actions, &no_nature_behaviour, &costs_to_goal);
    }

    println!("{:?}", states);
    println!("{:?}", no_nature_behaviour);
    println!("{:?}", costs_to_goal);
}
