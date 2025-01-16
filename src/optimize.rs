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

struct ProblemSolver<'p, P:Problem> {
    problem:&'p P,
    start_state:P::State,
    cost_cache:CostCache<P::State>,
    best_predecessors:HashMap<P::State,Vec<(P::State,P::Action)>>
}

impl<P:Problem> ProblemSolver<'_, P> {

    fn new(problem:&P, start_state:P::State) -> ProblemSolver<P> {
        ProblemSolver { problem, start_state, cost_cache:CostCache::new(), best_predecessors:HashMap::new() }
    }

    // from an unordered list of states, extract one with minimal cost
    fn extract_cheapest_state(&self, backlog:&mut Vec<P::State>) -> (P::State, Cost) {
        let min_cost = backlog.iter().map(|state| *self.cost_cache.get(state).unwrap()).min().unwrap();
        let min_index = backlog.iter().position(|state| *self.cost_cache.get(state).unwrap() == min_cost).unwrap();
        let state = backlog.swap_remove(min_index);
        (state, min_cost)
    }

    fn find_best_path_to_end(&mut self) -> Option<(/* endstate: */P::State, /* cost: */Cost)> where <P as Problem>::Action: 'static {

        // these states are to investigate
        let mut backlog:Vec<P::State> = Vec::new();

        self.cost_cache.insert(self.start_state, 0);
        backlog.push(self.start_state);

        // recursion termination at start point
        if self.problem.is_end_state(&self.start_state)  {
            if VERBOSE { println!("Terminated at start");}
            return Some((self.start_state, 0));
        }

        while backlog.len() > 0 {
            // extract element with minimum cost
            let (state, current_cost) = self.extract_cheapest_state(&mut backlog);
            if VERBOSE { println!("Handle {:?} with cost = {}", state, current_cost);}

            for &action in P::Action::all_actions() {
                if VERBOSE { println!("  try to do {:?}", action);}
                if let Some(after) = self.problem.execute_action(state, action) {
                    let new_cost = action.cost();
                    assert!(new_cost > 0);
                    let cost_this_way = new_cost + current_cost;

                    // recursion termination
                    if self.problem.is_end_state(&after) {
                        if VERBOSE { println!("Terminated");}
                        // not yet implemented in a generic way
                        //if VERBOSE { self.print_cache(cache);}
                        return Some((after, cost_this_way));
                    }

                    if let Some(&best_cost_up_to_now) = self.cost_cache.get(&after) {
                        if cost_this_way < best_cost_up_to_now {
                            self.cost_cache.insert(after, cost_this_way);
                            if VERBOSE { println!("  better cost for {:?}: {} < {}", after, cost_this_way, best_cost_up_to_now)}
                            backlog.push(after);
                        }
                    } else {
                        self.cost_cache.insert(after, cost_this_way);
                        if VERBOSE { println!("  cost for {:?}: {}", after, cost_this_way)}
                        backlog.push(after);
                    }
                }
            }
        }

        return None;
    }

    // find all best path and fill predecessor cache
    fn find_all_best_path_to_end_states(&mut self, maximal_cost:Cost) -> Vec<(/* endstate: */P::State, /* cost: */Cost)> where <P as Problem>::Action: 'static {

        // these states are to investigate
        let mut backlog:Vec<P::State> = Vec::new();
        let mut end_states:HashMap<P::State, Cost> = HashMap::new();

        self.cost_cache.insert(self.start_state, 0);
        backlog.push(self.start_state);

        // recursion termination at start point
        if self.problem.is_end_state(&self.start_state)  {
            if VERBOSE { println!("Terminated at start");}
            return vec![(self.start_state, 0)];
        }

        while backlog.len() > 0 {
            // extract element with minimum cost
            let (state, current_cost) = self.extract_cheapest_state(&mut backlog);
            if VERBOSE { println!("Handle {:?} with cost = {}", state, current_cost);}

            for &action in P::Action::all_actions() {
                if VERBOSE { println!("  try to do {:?}", action);}
                if let Some(after) = self.problem.execute_action(state, action) {
                    let new_cost = action.cost();
                    assert!(new_cost > 0);
                    let cost_this_way = new_cost + current_cost;

                    if cost_this_way > maximal_cost {
                        if VERBOSE { println!("Ignore because cost {} > max cost {}", cost_this_way, maximal_cost);}
                        break;
                    }

                    if let Some(&best_cost_up_to_now) = self.cost_cache.get(&after) {
                        if cost_this_way < best_cost_up_to_now {
                            self.cost_cache.insert(after, cost_this_way);
                            self.best_predecessors.insert(after, vec![(state, action)]);
                            if VERBOSE { println!("  better cost for {:?}: {} < {}", after, cost_this_way, best_cost_up_to_now)}

                            if self.problem.is_end_state(&after) {
                                if VERBOSE { println!("Found cheaper way to new end state {:?} with cost {}", after, cost_this_way);}
                                end_states.insert(after, cost_this_way);
                            } else {
                                backlog.push(after);
                            }
                        } else if cost_this_way == best_cost_up_to_now {
                            self.best_predecessors.get_mut(&after).unwrap().push((state, action));
                            if VERBOSE { println!("  equal cost for {:?}: {}", after, cost_this_way)}
                            assert!(self.cost_cache.contains_key(&after) || backlog.contains(&after));
                        }
                    } else {
                        self.cost_cache.insert(after, cost_this_way);
                        self.best_predecessors.insert(after, vec![(state, action)]);
                        if VERBOSE { println!("  first cost for {:?}: {}", after, cost_this_way)}
                        if self.problem.is_end_state(&after) {
                            if VERBOSE { println!("Found new end state {:?} with cost {}", after, cost_this_way);}
                            end_states.insert(after, cost_this_way);
                        } else {
                            backlog.push(after);
                        }
                    }
                }
            }
        }

        // convert hash map to vector
        end_states.iter().map(|(&state,&cost)|(state,cost)).collect::<Vec<(/* endstate: */P::State, /* cost: */Cost)>>()
    }
}

// find one path with lowest cost to an end state
pub fn get_cost_of_state<P:Problem>(problem:&P, start_state:P::State) -> Cost where <P as Problem>::Action: 'static {

    let mut solver = ProblemSolver::new(problem, start_state);

    match solver.find_best_path_to_end() {
        Some((_, cost)) => cost,
        None => {
            if VERBOSE { println!("Did not find any path to the end from {:?}", start_state); }
            u32::MAX
        }
    }
}

#[cfg(test)]
pub fn get_cost_cache<P:Problem>(problem:&P, start_state:P::State) -> CostCache<P::State> where <P as Problem>::Action: 'static {

    let mut solver = ProblemSolver::new(problem, start_state);

    solver.find_best_path_to_end();

    solver.cost_cache.clone()

}


#[cfg(test)]
mod test {

use crate::optimize::{get_cost_cache, get_cost_of_state, Problem, ProblemSolver};

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

    let cost_cache = get_cost_cache(&problem, TestState{value:1});
    assert_eq!(cost_cache.get(&TestState{value:1}), Some(&0));
    assert_eq!(cost_cache.get(&TestState{value:2}), Some(&1));
    assert_eq!(cost_cache.get(&TestState{value:3}), Some(&2));
    assert_eq!(cost_cache.get(&TestState{value:4}), Some(&2));

}

#[test]
fn test_all_paths() {
    let problem = TestProblem {target_value:5};
    let mut solver = ProblemSolver::new(&problem, TestState{value:1});
    let endstates = solver.find_all_best_path_to_end_states(10);
    assert_eq!(endstates, vec![(TestState{value:5},3)]);
    assert_eq!(endstates.len(), 1);
    let (endstate, endcost) = endstates[0];
    assert_eq!(endstate, TestState{value:5});
    assert_eq!(endcost, 3);
 
    assert_eq!(solver.cost_cache.get(&TestState{value:2}).unwrap(),&1);
    assert_eq!(solver.best_predecessors.get(&TestState{value:2}).unwrap(),&vec![(TestState{value:1}, TestAction::Double), (TestState{value:1}, TestAction::Increment)]);
    assert_eq!(solver.cost_cache.get(&TestState{value:3}).unwrap(),&2);
    assert_eq!(solver.best_predecessors.get(&TestState{value:3}).unwrap(),&vec![(TestState{value:2}, TestAction::Increment)]);
    assert_eq!(solver.cost_cache.get(&TestState{value:4}).unwrap(),&2);
    assert_eq!(solver.best_predecessors.get(&TestState{value:4}).unwrap(),&vec![(TestState{value:2}, TestAction::Double)]);
    assert_eq!(solver.cost_cache.get(&TestState{value:5}).unwrap(),&3);
    assert_eq!(solver.best_predecessors.get(&TestState{value:5}).unwrap(),&vec![(TestState{value:4}, TestAction::Increment)]);
}

}