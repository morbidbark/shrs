use std::{cell::RefCell, collections::VecDeque};

use crate::{
    prelude::{HookCtx, Shell},
    state::States,
};

pub trait Command: Send + 'static {
    fn apply(&self, sh: &mut Shell, states: &mut States);
}

impl<F> Command for F
where
    F: Fn(&mut Shell, &mut States) + Send + 'static,
{
    fn apply(&self, sh: &mut Shell, states: &mut States) {
        self(sh, states);
    }
}

pub struct Commands {
    pub queue: RefCell<VecDeque<Box<dyn Command>>>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            queue: RefCell::new(VecDeque::new()),
        }
    }

    pub fn run<C: Command + 'static>(&self, command: C) {
        self.queue.borrow_mut().push_back(Box::new(command));
    }

    pub fn drain(&self, states: &States) -> VecDeque<Box<dyn Command>> {
        self.queue.borrow_mut().drain(..).collect()
    }
}
