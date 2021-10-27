use stateright::*;

#[derive(Debug, Clone)]
pub enum Action {
    Read(usize),
    Write(usize),
}

#[derive(Debug, Clone, Default, Hash)]
pub struct State {
    i: u8,
    ts: Vec<u8>,
    pcs: Vec<u8>,
}

impl State {
    pub fn new(n: usize) -> Self {
        Self {
            i: 0,
            ts: vec![0; n],
            pcs: vec![1; n],
        }
    }
}

impl Model for State {
    type State = State;
    type Action = Action;

    fn init_states(&self) -> std::vec::Vec<<Self as stateright::Model>::State> {
        vec![self.clone()]
    }

    fn actions(&self, _state: &Self::State, actions: &mut Vec<Self::Action>) {
        actions.extend(&mut (0..self.pcs.len()).map(Action::Read));
        actions.extend(&mut (0..self.pcs.len()).map(Action::Write));
    }

    fn next_state(&self, last_state: &Self::State, action: Self::Action) -> Option<Self::State> {
        match action {
            Action::Read(n) => {
                if last_state.pcs[n] == 1 {
                    let mut state = last_state.clone();
                    state.ts[n] = last_state.i;
                    state.pcs[n] = 2;
                    Some(state)
                } else {
                    None
                }
            }
            Action::Write(n) => {
                if last_state.pcs[n] == 2 {
                    let mut state = last_state.clone();
                    state.pcs[n] = 3;
                    state.i = last_state.ts[n] + 1;
                    Some(state)
                } else {
                    None
                }
            }
        }
    }

    fn properties(&self) -> Vec<Property<Self>> {
        vec![Property::<Self>::eventually("fin", |_, state| {
            state.pcs.iter().all(|&pc| pc == 3) && state.i == (state.pcs.len() as u8)
        })]
    }
}
