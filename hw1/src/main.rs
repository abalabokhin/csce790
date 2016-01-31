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
            for (ref nature_action, ref probability) in nature_behabiour {
                let new_state = do_action(&state, &action, &nature_action, &available_states);
                let position = available_states.iter().position(|ref x| x.i == new_state.i && x.j == new_state.j).unwrap();
                let contribution = *probability * (costs_to_goal[position] + 1.);
                current_cost_to_goal += contribution;
            }
            if current_cost_to_goal < min_cost_to_goal {
                min_cost_to_goal = current_cost_to_goal;
            }
        }
        new_costs_to_goal.push(min_cost_to_goal);
    }
    return new_costs_to_goal;
}

fn are_arrays_different(a1 : &Vec<f32>, a2 : &Vec<f32>, epsilon: f32)->bool {
    for i in 0..a1.len() {
        if (a1[i] - a2[i]).abs() > epsilon {
            return true;
        }
    }
    return false;
}

fn build_plan_based_on_costs_to_goal(available_states : &Vec<State>, robot_actions : &Vec<Action>, costs_to_goal: &Vec<f32>)->Vec<Action> {
    return vec![];
}

fn build_optimal_plan(available_states : &Vec<State>, robot_actions : &Vec<Action>, nature_behaviour : &HashMap<Action, f32>)->(i32, Vec<f32>, Vec<Action>) {
    let mut costs_to_goal = available_states.iter().map(|ref x| if is_goal(x) {0.} else {INFINITY}).collect::<Vec<f32>>();    
    let mut x = 0;
    loop {
        let new_costs_to_goal = recalculate_cost_to_goals(&available_states, &robot_actions, &nature_behaviour, &costs_to_goal);
        if !are_arrays_different(&new_costs_to_goal, &costs_to_goal, 0.1) {
            break;
        }
        costs_to_goal = new_costs_to_goal;
        x += 1;
    }
    let plan = build_plan_based_on_costs_to_goal(&available_states, &robot_actions, &costs_to_goal);
    return (x, costs_to_goal, plan);
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

    let robot_actions = vec![Action{di : 0, dj : 0}, Action{di : 1, dj : 0}, Action{di : 0, dj : 1}, Action{di : -1, dj : 0}, Action{di : 0, dj : -1}];
    let mut no_nature_behaviour = HashMap::new();
    no_nature_behaviour.insert(Action{di : 0, dj : 0}, 1.);

    let costs_to_goal = build_optimal_plan(&states, &robot_actions, &no_nature_behaviour);

    println!("{:?}", states);
    println!("{:?}", no_nature_behaviour);
    println!("{:?}", costs_to_goal);
}
