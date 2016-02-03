extern crate rand;

use std::collections::BTreeMap;
use std::collections::HashMap;
use rand::distributions::{IndependentSample, Range};

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
    nature_behaviour : &HashMap<Action, f32>, 
    costs_to_goal : &Vec<f32>)->Vec<f32> {
    
    let mut new_costs_to_goal = vec![];
    for ref state in available_states {
        let mut min_cost_to_goal = if is_goal(state) {0.} else {INFINITY}; 
        for ref action in robot_actions {
            let mut current_cost_to_goal = 0.;
            for (ref nature_action, ref probability) in nature_behaviour {
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
    let mut plan = vec![];
    for ref state in available_states {
        let mut min_cost_to_goal = if is_goal(state) {0.} else {INFINITY};
        let mut best_action = Action{di: 0, dj: 0};
        for ref action in robot_actions {
            let new_state = do_action(&state, &action, &Action{di: 0, dj: 0}, &available_states);
            let position = available_states.iter().position(|ref x| x.i == new_state.i && x.j == new_state.j).unwrap();
            if costs_to_goal[position] + 1. < min_cost_to_goal {
                min_cost_to_goal = costs_to_goal[position] + 1.;
                best_action = Action{di: action.di, dj: action.dj};
            }
        }
        plan.push(best_action);
    }

    return plan; 
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

fn print_costs(available_states : &Vec<State>, costs_to_goal: &Vec<f32>) {
    println!("Optimal costs to goal:");
    for y in 1..W + 1 {
        for x in 1..W + 1 {
            match available_states.iter().position(|ref pos| pos.i == x && pos.j == y) {
                Some(v)     => { let formatted_number = format!("{:.*}", 2, costs_to_goal[v]); print!("{}\t", formatted_number); }
                None        => print!("\t")
            }
        }
        println!("");
    }
}


fn print_plan(available_states : &Vec<State>, plan : &Vec<Action>) {
    println!("Optimal action plan:");
    for y in 1..W + 1 {
        for x in 1..W + 1 {
            match available_states.iter().position(|ref pos| pos.i == x && pos.j == y) {
                Some(v)     => print!("{},{}\t", plan[v].di, plan[v].dj),
                None        => print!("\t")
            }
        }
        println!("");
    }
}

fn choose_nature_action_randomly(nature_behaviour : &HashMap<Action, f32>)->Action {
    let between = Range::new(0f32, 1.);
    let mut rng = rand::thread_rng();
    let a = between.ind_sample(&mut rng);
    let mut sum = 0f32;
    for (ref nature_action, ref probability) in nature_behaviour {
        sum += **probability;
        if sum >= a {
            return Action{di: nature_action.di, dj: nature_action.dj};
        }
    }
    return Action{di: 0, dj: 0};
}

fn simulate_robot(nature_behaviour : &HashMap<Action, f32>, start_state : &State, available_states : &Vec<State>, plan : &Vec<Action>)->i32 {
    let mut current_state = State{i: start_state.i, j: start_state.j};
    let mut total_cost = 0;
    loop {
        let nature_action = choose_nature_action_randomly(&nature_behaviour);
        let action = &plan[available_states.iter().position(|ref x| x.i == current_state.i && x.j == current_state.j).unwrap()];
        current_state = do_action(&current_state, action, &nature_action, available_states);
        total_cost += 1;
        if is_goal(&current_state) {
            return total_cost;
        }
    }
}

fn main() {
    let mut states = vec![]; 
    // Creating all states
    for y in 1..W + 1 {
        for x in 1..W + 1 {
            if x <= W / 3 || x > 2 * W / 3 || y <= W / 3 || y > 2 * W / 3 {
                states.push(State {i: x, j : y}); 
            }
        }
    }

    let robot_actions = vec![Action{di : 0, dj : 0}, Action{di : 1, dj : 0}, Action{di : 0, dj : 1}, Action{di : -1, dj : 0}, Action{di : 0, dj : -1}];
    let mut no_nature_behaviour = HashMap::new();
    no_nature_behaviour.insert(Action{di : 0, dj : 0}, 1.);

    let plan = build_optimal_plan(&states, &robot_actions, &no_nature_behaviour);
     
    println!("1. Iterations to converge: {}", plan.0);
    println!("2. If the nature can choose the worst case every time, it can prevent us from reaching the goal from every, not goal position. Thus, the cost function would be infinity in any state except goal ones.
    However if the nature can push us only right and bottom, the worst case nature strategy would be to do nothing. The algorithm is the same as if we have no nature at all, see 1).");

    let mut nature_behaviour = HashMap::new();
    nature_behaviour.insert(Action{di : 0, dj : 0}, 0.96);
    nature_behaviour.insert(Action{di : 1, dj : 0}, 0.01);
    nature_behaviour.insert(Action{di : 0, dj : 1}, 0.01);
    nature_behaviour.insert(Action{di : -1, dj : 0}, 0.01);
    nature_behaviour.insert(Action{di : 0, dj : -1}, 0.01);

    let plan2 = build_optimal_plan(&states, &robot_actions, &nature_behaviour);
    
    println!(3.);
    print_costs(&states, &plan2.1);
    print_plan(&states, &plan2.2);

    let mut distribution = BTreeMap::new();
    let mut average_cost = 0.;
    for _ in 1..1000 {
        let cost = simulate_robot(&nature_behaviour, &State{i:1, j:1}, &states, &plan2.2);
        if distribution.contains_key(&cost) {
            *distribution.get_mut(&cost).unwrap() += 1;
        } else {
            distribution.insert(cost, 1);
        }
        average_cost += cost as f32 / 1000.;
    }
    println!("4. Cost distribution in 1000 samples is: {:?}", distribution);
    println!("\tAverage cost is {}.", average_cost);
    println!("\tAerage cost is really close to computed cost");

    println!("5. Fun with code is not shown in the code. I did some experiments by changing code and then changed everything back. However the answers:
    1. Even if W = 60, it takes about 20 seconds to wait for the convergence with some nature actions. With 90 I didn't wait long enough. The algorithm can be optimized, though. Now T(n) = O(W^4), but it can be optimized to O(W^3), if I calculated it right.
    2. If the probability of Qk = 0 is close two zero in doesn't change anything much. Cause the nature has an equal chance to help robot and to move it back
    3. If the probability of Qk is close to 1, the result is close to the problem in 1. Actually the problem in 1 was done by using this probability equal to 1.");
}
