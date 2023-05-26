use crate::STATE;
use crate::types::candid::{ControllerId, Controllers, Entity};

pub fn set_controllers(id: &ControllerId, controller: &Entity) {
    STATE.with(|state| {
        set_controllers_impl(
            id,
            controller,
            &mut state.borrow_mut().stable.controllers,
        )
    })
}

pub fn set_controllers_impl(
    id: &ControllerId,
    controller: &Entity,
    controllers: &mut Controllers,
) {
    controllers.insert(*id, controller.clone());
}