import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Controller { 'updated_at' : bigint, 'created_at' : bigint }
export interface Entity {
  'updated_at' : bigint,
  'data' : Uint8Array | number[],
  'created_at' : bigint,
}
export interface _SERVICE {
  'del_stable_entity' : ActorMethod<[string, string], undefined>,
  'get_candid_controllers' : ActorMethod<[], Array<[Principal, Controller]>>,
  'get_candid_entity' : ActorMethod<[string, string], [] | [Entity]>,
  'get_stable_controllers' : ActorMethod<[], Array<[Principal, Controller]>>,
  'get_stable_entity' : ActorMethod<[string, string], [] | [Entity]>,
  'set_candid_controllers' : ActorMethod<[Principal, Controller], undefined>,
  'set_candid_entity' : ActorMethod<[string, string, Entity], undefined>,
  'set_stable_controllers' : ActorMethod<[Principal, Controller], undefined>,
  'set_stable_entity' : ActorMethod<[string, string, Entity], undefined>,
}
