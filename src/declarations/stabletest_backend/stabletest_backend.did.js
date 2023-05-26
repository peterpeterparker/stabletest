export const idlFactory = ({ IDL }) => {
  const Entity = IDL.Record({
    'updated_at' : IDL.Nat64,
    'created_at' : IDL.Nat64,
  });
  return IDL.Service({
    'get_candid_controllers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Entity))],
        ['query'],
      ),
    'get_stable_controllers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Entity))],
        ['query'],
      ),
    'set_candid_controllers' : IDL.Func([IDL.Principal, Entity], [], []),
    'set_stable_controllers' : IDL.Func([IDL.Principal, Entity], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
