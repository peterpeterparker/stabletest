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
const key = "my_key";

const addCandidEntity = async () => {
  const entity = {
    data: [2],
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_candid_entity(collection, key, entity);

  await getCandidEntity();
}

const getCandidEntity = async () => console.log(await stabletest_backend.get_candid_entity(collection, key));

const addStableEntity = async () => {
  const entity = {
    data: [1],
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_stable_entity(collection, key, entity);

  await getStableEntity();
}

const getStableEntity = async () => console.log(await stabletest_backend.get_stable_entity(collection, key));

const delStableEntity = async () => {
  await stabletest_backend.del_stable_entity(collection, key);

  await getStableEntity();
}

// Init

const init = () => {
  document.querySelector("#add-candid-controllers").addEventListener("click", addCandidControllers, {passive: true});
  document.querySelector("#get-candid-controllers").addEventListener("click", getCandidControllers, {passive: true});
  document.querySelector("#add-stable-controllers").addEventListener("click", addStableControllers, {passive: true});
  document.querySelector("#get-stable-controllers").addEventListener("click", getStableControllers, {passive: true});

  document.querySelector("#add-candid-entity").addEventListener("click", addCandidEntity, {passive: true});
  document.querySelector("#get-candid-entity").addEventListener("click", getCandidEntity, {passive: true});
  document.querySelector("#add-stable-entity").addEventListener("click", addStableEntity, {passive: true});
  document.querySelector("#get-stable-entity").addEventListener("click", getStableEntity, {passive: true});
  document.querySelector("#del-stable-entity").addEventListener("click", delStableEntity, {passive: true});
}

document.addEventListener("DOMContentLoaded", init, {once: true});