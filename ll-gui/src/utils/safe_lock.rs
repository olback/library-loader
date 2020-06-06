use crate::types::AMState;

pub fn safe_lock<F, T>(state: &AMState, f: F) -> T
    where F: FnOnce(&mut std::sync::MutexGuard<'_, crate::state::State>) -> T {

    let l_state = state.clone();

    let r: T;

    match l_state.try_lock().as_mut() {

        Ok(lock) => {
            r = f(lock);
            drop(lock);
        },
        Err(_) => {
            unreachable!("Deadlock!");
        }

    }

    r

}
