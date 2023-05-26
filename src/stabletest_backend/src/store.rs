use crate::types::candid::{ControllerId, Controllers, Entity};
use crate::types::stable::MyPrincipal;
use crate::{CANDID_STATE, CONTROLLERS_STATE, STABLE_STATE};

/// Candid

pub fn set_candid_controllers(id: &ControllerId, controller: &Entity) {
    CANDID_STATE.with(|state| {
        set_candid_controllers_impl(id, controller, &mut state.borrow_mut().stable.controllers)
    })
}

pub fn set_candid_controllers_impl(
    id: &ControllerId,
    controller: &Entity,
    controllers: &mut Controllers,
) {
    controllers.insert(*id, controller.clone());
}

pub fn get_candid_controllers() -> Controllers {
    CANDID_STATE.with(|state| state.borrow().stable.controllers.clone())
}

/// Stable

pub fn set_stable_controllers(key: &MyPrincipal, controller: &Entity) {
    CONTROLLERS_STATE.with(|p| p.borrow_mut().insert(key.clone(), controller.clone()));
}

pub fn get_stable_controllers() -> Vec<(MyPrincipal, Entity)> {
    CONTROLLERS_STATE.with(|p| p.borrow().iter().collect())
}
