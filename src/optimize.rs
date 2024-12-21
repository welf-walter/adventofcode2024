
pub trait Problem {
    type State:std::fmt::Debug + PartialEq;
    type Action:Sized;
    fn all_actions() -> &'static [Self::Action];
    // what happens if an action is executed on before state?
    // return None if action is invalid
    fn execute_action(&self, before:Self::State, action:Self::Action) -> Option<Self::State>;
}

#[cfg(test)]
mod test {

use crate::optimize::Problem;

#[derive(Debug, PartialEq)]
struct TestState {
    value:u32
}

#[derive(Debug, PartialEq)]
enum TestAction {
    Double,
    Increment,
    Decrement
}
struct TestProblem {

    
}

impl Problem for TestProblem {
    type State = TestState;
    type Action = TestAction;

    fn all_actions() -> &'static [TestAction] {
        &[TestAction::Double, TestAction::Increment, TestAction::Decrement]
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
    assert_eq!(TestProblem::all_actions().len(), 3);
    assert_eq!(TestProblem::all_actions()[1], TestAction::Increment);
    let problem = TestProblem {};
    assert_eq!(problem.execute_action(TestState{value:5}, TestAction::Double), Some(TestState{value:10}));
    assert_eq!(problem.execute_action(TestState{value:5}, TestAction::Increment), Some(TestState{value:6}));
    assert_eq!(problem.execute_action(TestState{value:0}, TestAction::Decrement), None);
}

}