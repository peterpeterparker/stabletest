import { stabletest_backend } from "../../declarations/stabletest_backend";
import {AnonymousIdentity} from "@dfinity/agent";

const addCandid = async () => {
  const controller = {
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_candid_controllers(new AnonymousIdentity().getPrincipal(), controller);

  await getCandid();
}

const getCandid = async () => console.log(await stabletest_backend.get_candid_controllers());


const addStable = async () => {
  const controller = {
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_stable_controllers(new AnonymousIdentity().getPrincipal(), controller);

  await getStable();
}

const getStable = async () => console.log(await stabletest_backend.get_stable_controllers());

const init = () => {
  document.querySelector("#add-candid").addEventListener("click", addCandid, {passive: true});
  document.querySelector("#get-candid").addEventListener("click", getCandid, {passive: true});
  document.querySelector("#add-stable").addEventListener("click", addStable, {passive: true});
  document.querySelector("#get-stable").addEventListener("click", getStable, {passive: true});
}

document.addEventListener("DOMContentLoaded", init, {once: true});