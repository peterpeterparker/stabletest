export const idlFactory = ({ IDL }) => {
  const Controller = IDL.Record({
    'updated_at' : IDL.Nat64,
    'created_at' : IDL.Nat64,
  });
  const Entity = IDL.Record({
    'updated_at' : IDL.Nat64,
    'data' : IDL.Vec(IDL.Nat8),
    'created_at' : IDL.Nat64,
  });
  return IDL.Service({
    'get_candid_controllers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Controller))],
        ['query'],
      ),
    'get_candid_entities' : IDL.Func([IDL.Text], [IDL.Vec(Entity)], ['query']),
    'get_candid_entity' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Opt(Entity)],
        ['query'],
      ),
    'get_stable_entities' : IDL.Func([IDL.Text], [IDL.Vec(Entity)], ['query']),
    'get_stable_entity' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Opt(Entity)],
        ['query'],
      ),
    'set_candid_controllers' : IDL.Func([IDL.Principal, Controller], [], []),
    'set_candid_entity' : IDL.Func([IDL.Text, IDL.Text, Entity], [], []),
    'set_stable_entity' : IDL.Func([IDL.Text, IDL.Text, Entity], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
