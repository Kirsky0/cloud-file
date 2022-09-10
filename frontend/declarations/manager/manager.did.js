export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_store' : IDL.Func([], [IDL.Text], ['query']),
    'get_wasm' : IDL.Func([], [IDL.Text], ['query']),
    'mem_size' : IDL.Func([], [IDL.Nat64], ['query']),
    'upload_module' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Text], []),
    'upload_wasm' : IDL.Func([IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
