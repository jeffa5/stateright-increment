use stateright::*;

#[derive(Debug, Clone)]
pub enum Action {
    Read(usize),
    Write(usize),
    Lock(usize),
    Unlock(usize),
}

#[derive(Debug, Clone, Default, Hash)]
pub struct State {
    i: u8,
    ts: Vec<u8>,
    pcs: Vec<u8>,
    lock: bool,
}

impl State {
    pub fn new(n: usize) -> Self {
        Self {
            i: 0,
            ts: vec![0; n],
            pcs: vec![0; n],
            lock: false,
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
        actions.extend(&mut (0..self.pcs.len()).map(Action::Lock));
        actions.extend(&mut (0..self.pcs.len()).map(Action::Unlock));
    }

    fn next_state(&self, last_state: &Self::State, action: Self::Action) -> Option<Self::State> {
        match action {
            Action::Lock(n) => {
                if last_state.pcs[n] == 0 && !last_state.lock {
                    let mut pcs = last_state.pcs.clone();
                    pcs[n] = 1;
                    Some(State {
                        pcs,
                        lock: true,
                        ..last_state.clone()
                    })
                } else {
                    None
                }
            }
            Action::Read(n) => {
                if last_state.pcs[n] == 1 {
                    let mut ts = last_state.ts.clone();
                    ts[n] = last_state.i;
                    let mut pcs = last_state.pcs.clone();
                    pcs[n] = 2;
                    Some(State {
                        ts,
                        pcs,
                        ..last_state.clone()
                    })
                } else {
                    None
                }
            }
            Action::Write(n) => {
                if last_state.pcs[n] == 2 {
                    let mut pcs = last_state.pcs.clone();
                    pcs[n] = 3;
                    Some(State {
                        i: last_state.ts[n] + 1,
                        pcs,
                        ..last_state.clone()
                    })
                } else {
                    None
                }
            }
            Action::Unlock(n) => {
                if last_state.pcs[n] == 3 && last_state.lock {
                    let mut pcs = last_state.pcs.clone();
                    pcs[n] = 4;
                    Some(State {
                        pcs,
                        lock: false,
                        ..last_state.clone()
                    })
                } else {
                    None
                }
            }
        }
    }

    fn properties(&self) -> Vec<Property<Self>> {
        vec![Property::<Self>::eventually("fin", |_, state| {
            state.pcs.iter().all(|&pc| pc == 4) && state.i == (state.pcs.len() as u8)
        })]
    }
}
