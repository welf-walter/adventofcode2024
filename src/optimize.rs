use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub type Cost = u32;

// enable for debugging
const VERBOSE:bool = false;

pub trait ActionTrait:Sized+Clone+Copy+Debug {
    fn all_actions() -> &'static [Self];
    fn cost(self) -> Cost;
}

pub trait Problem {
    type State:std::fmt::Debug + PartialEq + Hash + Eq + Copy;
    type Action:ActionTrait;

    // does this state solve the problem?
    fn is_end_state(&self, state:&Self::State) -> bool;

    // what happens if an action is executed on before state?
    // return None if action is invalid
    fn execute_action(&self, before:Self::State, action:Self::Action) -> Option<Self::State>;
}

// remember the currently lowest cost for a state
// not there means infinite
type CostCache<State> = HashMap<State,Cost>;

// from an unordered list of states, extract one with minimal cost
fn extract_cheapest_state<State:Hash+Eq>(backlog:&mut Vec<State>, cost_cache:&CostCache<State>) -> (State, Cost) {
    let min_cost = backlog.iter().map(|state| *cost_cache.get(state).unwrap()).min().unwrap();
    let min_index = backlog.iter().position(|state| *cost_cache.get(state).unwrap() == min_cost).unwrap();
    let state = backlog.swap_remove(min_index);
    (state, min_cost)
}

// find one path with lowest cost to an end state
pub fn get_cost_of_state<P:Problem>(problem:&P, start_state:P::State) -> Cost where <P as Problem>::Action: 'static {

    // these states are to investigate
    let mut backlog:Vec<P::State> = Vec::new();
    let mut cost_cache:CostCache<P::State> = HashMap::new();

    cost_cache.insert(start_state, 0);
    backlog.push(start_state);

    // recursion termination at start point
    if problem.is_end_state(&start_state)  {
        if VERBOSE { println!("Terminated at start");}
        return 0;
    }

    while backlog.len() > 0 {
        // extract element with minimum cost
        let (state, current_cost) = extract_cheapest_state(&mut backlog, &cost_cache);
        if VERBOSE { println!("Handle {:?} with cost = {}", state, current_cost);}

        for &action in P::Action::all_actions() {
            if VERBOSE { println!("  try to do {:?}", action);}
            if let Some(after) = problem.execute_action(state, action) {
                let cost_this_way = action.cost() + current_cost;

                // recursion termination
                if problem.is_end_state(&after) {
                    if VERBOSE { println!("Terminated");}
                    // not yet implemented in a generic way
                    //if VERBOSE { self.print_cache(cache);}
                    return cost_this_way;
                }

                if let Some(&best_cost_up_to_now) = cost_cache.get(&after) {
                    if cost_this_way < best_cost_up_to_now {
                        cost_cache.insert(after, cost_this_way);
                        if VERBOSE { println!("  better cost for {:?}: {} < {}", after, cost_this_way, best_cost_up_to_now)}
                        backlog.push(after);
                    }
                } else {
                    cost_cache.insert(after, cost_this_way);
                    if VERBOSE { println!("  cost for {:?}: {}", after, cost_this_way)}
                    backlog.push(after);
                }
            }
        }
    }

    panic!("Did not find any path to the end");
    }


#[cfg(test)]
mod test {

use crate::optimize::{get_cost_of_state, Problem};

use super::ActionTrait;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct TestState {
    value:u32
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TestAction {
    Double,
    Increment,
    Decrement
}

impl ActionTrait for TestAction {
    fn all_actions() -> &'static [TestAction] {
        &[TestAction::Double, TestAction::Increment, TestAction::Decrement]
    }
    fn cost(self) -> super::Cost { 1 }
}

struct TestProblem {
    target_value:u32
}

impl Problem for TestProblem {
    type State = TestState;
    type Action = TestAction;

    fn is_end_state(&self, state:&Self::State) -> bool {
        state.value == self.target_value
    }

    fn execute_action(&self, before:Self::State, action:Self::Action) -> Option<Self::State> {
        match action {
            TestAction::Double    => { Some(Self::State{value: before.value * 2})},
            TestAction::Increment => { Some(Self::State{value: before.value + 1})},
            TestAction::Decrement => { if before.value > 0 { Some(Self::State{value: before.value - 1})} else { None } }
        }
    }

}

#[test]
fn test_actions() {
    assert_eq!(TestAction::all_actions().len(), 3);
    assert_eq!(TestAction::all_actions()[1], TestAction::Increment);
    let problem = TestProblem {target_value:15};
    assert_eq!(problem.execute_action(TestState{value:5}, TestAction::Double), Some(TestState{value:10}));
    assert_eq!(problem.execute_action(TestState{value:5}, TestAction::Increment), Some(TestState{value:6}));
    assert_eq!(problem.execute_action(TestState{value:0}, TestAction::Decrement), None);
    assert_eq!(TestAction::Decrement.cost(), 1);
    // expected best solution: Double/Increment, Double, Double, Double, Decrement
    assert_eq!(get_cost_of_state(&problem, TestState{value:1}), 5);
}

}