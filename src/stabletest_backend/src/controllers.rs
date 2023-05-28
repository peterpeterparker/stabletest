use crate::types::candid::{Controller, ControllerId, Controllers};
use crate::types::stable::MyPrincipal;
use crate::{CANDID_STATE, CONTROLLERS_STATE};

/// Candid

pub fn set_candid_controllers(id: &ControllerId, controller: &Controller) {
    CANDID_STATE.with(|state| {
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
    CANDID_STATE.with(|state| state.borrow().stable.controllers.clone())
}

/// Stable

pub fn set_stable_controllers(key: &MyPrincipal, controller: &Controller) {
    CONTROLLERS_STATE.with(|p| p.borrow_mut().insert(key.clone(), controller.clone()));
}

pub fn get_stable_controllers() -> Vec<(MyPrincipal, Controller)> {
    CONTROLLERS_STATE.with(|p| p.borrow().iter().collect())
}
