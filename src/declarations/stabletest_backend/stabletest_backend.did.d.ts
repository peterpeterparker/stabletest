import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Entity { 'updated_at' : bigint, 'created_at' : bigint }
export interface _SERVICE {
  'get_candid_controllers' : ActorMethod<[], Array<[Principal, Entity]>>,
  'get_stable_controllers' : ActorMethod<[], Array<[Principal, Entity]>>,
  'set_candid_controllers' : ActorMethod<[Principal, Entity], undefined>,
  'set_stable_controllers' : ActorMethod<[Principal, Entity], undefined>,
}
