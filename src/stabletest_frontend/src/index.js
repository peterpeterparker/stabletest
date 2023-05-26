import { stabletest_backend } from "../../declarations/stabletest_backend";
import {AnonymousIdentity} from "@dfinity/agent";

const action = async () => {
  const controller = {
    created_at: 0n,
    updated_at: 0n,
  };

  await stabletest_backend.set_controllers(new AnonymousIdentity().getPrincipal(), controller);

  console.log(await stabletest_backend.get_controllers())
}

const init = () => document.querySelector("button").addEventListener("click", action, {passive: true});

document.addEventListener("DOMContentLoaded", init, {once: true});