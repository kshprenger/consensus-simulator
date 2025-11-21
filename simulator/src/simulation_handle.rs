use std::cell::RefCell;

use crate::simulation::Simulation;

thread_local! {
    pub static SIMULATION_HANDLE: RefCell<Option<Simulation>> = RefCell::new(None);
}

pub(crate) fn setup_sim(sim: Simulation) {
    SIMULATION_HANDLE.set(Some(sim));
}

pub(crate) fn with_sim<F, R>(f: F) -> R
where
    F: FnOnce(&mut Simulation) -> R,
{
    SIMULATION_HANDLE.with(|cell| {
        let mut ref_mut = cell.borrow_mut();
        let sim = ref_mut.as_mut().expect("Out of simulation context");
        f(sim)
    })
}
