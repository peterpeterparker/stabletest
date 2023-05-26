import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Entity { 'updated_at' : bigint, 'created_at' : bigint }
export interface _SERVICE {
  'get_controllers' : ActorMethod<[], Array<[Principal, Entity]>>,
  'greet' : ActorMethod<[string], string>,
  'set_controllers' : ActorMethod<[Principal, Entity], undefined>,
}
