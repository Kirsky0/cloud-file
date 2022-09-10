export const idlFactory = ({ IDL }) => {
  const Status = IDL.Variant({
    'stopped' : IDL.Null,
    'stopping' : IDL.Null,
    'running' : IDL.Null,
  });
  const DefiniteCanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Nat,
    'controllers' : IDL.Vec(IDL.Principal),
    'memory_allocation' : IDL.Nat,
    'compute_allocation' : IDL.Nat,
  });
  const CanisterStatus = IDL.Record({
    'status' : Status,
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : DefiniteCanisterSettings,
    'idle_cycles_burned_per_day' : IDL.Nat,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  return IDL.Service({
    'mem_size' : IDL.Func([], [IDL.Nat64], ['query']),
    'store_status' : IDL.Func([IDL.Text], [CanisterStatus], []),
    'upload_module' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
