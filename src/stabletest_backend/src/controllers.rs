use crate::memory::STATE;
use crate::types::candid::{Controller, ControllerId, Controllers};

/// Candid

pub fn set_candid_controllers(id: &ControllerId, controller: &Controller) {
    STATE.with(|state| {
        set_candid_controllers_impl(id, controller, &mut state.borrow_mut().stable.controllers)
    })
}

pub fn set_candid_controllers_impl(
    id: &ControllerId,
    controller: &Controller,
    controllers: &mut Controllers,
) {
    controllers.insert(*id, controller.clone());
}

pub fn get_candid_controllers() -> Controllers {
    STATE.with(|state| state.borrow().stable.controllers.clone())
}
