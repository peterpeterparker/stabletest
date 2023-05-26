export const idlFactory = ({ IDL }) => {
  const Entity = IDL.Record({
    'updated_at' : IDL.Nat64,
    'created_at' : IDL.Nat64,
  });
  return IDL.Service({
    'get_controllers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Entity))],
        ['query'],
      ),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'set_controllers' : IDL.Func([IDL.Principal, Entity], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
