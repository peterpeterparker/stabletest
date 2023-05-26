import { stabletest_backend } from "../../declarations/stabletest_backend";
import {AnonymousIdentity} from "@dfinity/agent";

const add = async () => {
  const controller = {
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_controllers(new AnonymousIdentity().getPrincipal(), controller);

  await get();
}

const get = async () => console.log(await stabletest_backend.get_controllers());

const init = () => {
  document.querySelector("#add").addEventListener("click", add, {passive: true});
  document.querySelector("#get").addEventListener("click", get, {passive: true});
}

document.addEventListener("DOMContentLoaded", init, {once: true});