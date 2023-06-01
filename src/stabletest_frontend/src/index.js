import { stabletest_backend } from "../../declarations/stabletest_backend";
import {AnonymousIdentity} from "@dfinity/agent";

// Controllers

const addCandidControllers = async () => {
  const controller = {
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_candid_controllers(new AnonymousIdentity().getPrincipal(), controller);

  await getCandidControllers();
}

const getCandidControllers = async () => console.log(await stabletest_backend.get_candid_controllers());

const addStableControllers = async () => {
  const controller = {
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_stable_controllers(new AnonymousIdentity().getPrincipal(), controller);

  await getStableControllers();
}

const getStableControllers = async () => console.log(await stabletest_backend.get_stable_controllers());

// Entity

const collection = "my_collection";
const key = "my_key_3";

const addCandidEntity = async () => {
  const entity = {
    data: [5],
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_candid_entity(collection, key, entity);

  await getCandidEntity();
}

const getCandidEntity = async () => console.log(await stabletest_backend.get_candid_entity(collection, key));

const listCandidEntities = async () => console.log(await stabletest_backend.get_candid_entities(collection));

const addStableEntity = async () => {
  const entity = {
    data: [44],
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_stable_entity(collection, key, entity);

  await getStableEntity();
}

const getStableEntity = async () => console.log(await stabletest_backend.get_stable_entity(collection, key));

const listStableEntities = async () => console.log(await stabletest_backend.get_stable_entities(collection));

// Init

const init = () => {
  document.querySelector("#add-candid-controllers").addEventListener("click", addCandidControllers, {passive: true});
  document.querySelector("#get-candid-controllers").addEventListener("click", getCandidControllers, {passive: true});

  document.querySelector("#add-candid-entity").addEventListener("click", addCandidEntity, {passive: true});
  document.querySelector("#get-candid-entity").addEventListener("click", getCandidEntity, {passive: true});
  document.querySelector("#list-candid-entities").addEventListener("click", listCandidEntities, {passive: true});
  document.querySelector("#add-stable-entity").addEventListener("click", addStableEntity, {passive: true});
  document.querySelector("#get-stable-entity").addEventListener("click", getStableEntity, {passive: true});
  document.querySelector("#list-stable-entities").addEventListener("click", listStableEntities, {passive: true});
}

document.addEventListener("DOMContentLoaded", init, {once: true});