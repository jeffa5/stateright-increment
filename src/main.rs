use stateright::*;

#[derive(Debug, Clone)]
enum Action {
    Read,
    Write,
}

#[derive(Debug, Clone, Default, Hash)]
struct State {
    i: u8,
    t: u8,
    pc: u8,
}

impl Model for State {
    type State = State;
    type Action = Action;

    fn init_states(&self) -> std::vec::Vec<<Self as stateright::Model>::State> {
        vec![State { i: 0, t: 0, pc: 1 }]
    }

    fn actions(&self, _state: &Self::State, actions: &mut Vec<Self::Action>) {
        actions.append(&mut vec![Action::Read, Action::Write]);
    }

    fn next_state(&self, last_state: &Self::State, action: Self::Action) -> Option<Self::State> {
        match action {
            Action::Read if last_state.pc == 1 => Some(State {
                pc: 2,
                t: last_state.i,
                i: last_state.i,
            }),
            Action::Write if last_state.pc == 2 => Some(State {
                pc: 3,
                t: last_state.t,
                i: last_state.t + 1,
            }),
            _ => None,
        }
    }

    fn properties(&self) -> Vec<Property<Self>> {
        vec![Property::<Self>::eventually("fin", |_, state| {
            state.pc == 3 && state.i == 1
        })]
    }
}

fn main() {
    State::default().checker().serve("localhost:3000");
}
