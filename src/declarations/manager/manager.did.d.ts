import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'create_module' : ActorMethod<[string], string>,
  'get_store' : ActorMethod<[], string>,
  'get_wasm' : ActorMethod<[], string>,
  'mem_size' : ActorMethod<[], bigint>,
  'upload_wasm' : ActorMethod<[string], undefined>,
}
