pub type Cost = u32;

pub trait ActionTrait:Sized+Clone+Copy {
    fn all_actions() -> &'static [Self];
    fn cost(self) -> Cost;
}
pub trait Problem {
    type State:std::fmt::Debug + PartialEq;
    type Action:ActionTrait;
    // what happens if an action is executed on before state?
    // return None if action is invalid
    fn execute_action(&self, before:Self::State, action:Self::Action) -> Option<Self::State>;
}

#[cfg(test)]
mod test {

use crate::optimize::Problem;

use super::ActionTrait;

#[derive(Debug, PartialEq)]
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

}

impl Problem for TestProblem {
    type State = TestState;
    type Action = TestAction;

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
    let problem = TestProblem {};
    assert_eq!(problem.execute_action(TestState{value:5}, TestAction::Double), Some(TestState{value:10}));
    assert_eq!(problem.execute_action(TestState{value:5}, TestAction::Increment), Some(TestState{value:6}));
    assert_eq!(problem.execute_action(TestState{value:0}, TestAction::Decrement), None);
    assert_eq!(TestAction::Decrement.cost(), 1);
}

}