// Auto-generated declarations for auth canister
// This matches the auth.did file from your backend

export const idlFactory = ({ IDL }) => {
  const UserRole = IDL.Variant({
    'User': IDL.Null,
    'Admin': IDL.Null,
    'Moderator': IDL.Null,
  });

  const UserSession = IDL.Record({
    'user_principal': IDL.Principal,
    'github_username': IDL.Opt(IDL.Text),
    'created_at': IDL.Nat64,
    'last_active': IDL.Nat64,
    'expires_at': IDL.Nat64,
    'role': UserRole,
    'is_verified': IDL.Bool,
  });

  const AuthError = IDL.Variant({
    'NotAuthenticated': IDL.Null,
    'SessionExpired': IDL.Null,
    'InvalidPrincipal': IDL.Null,
    'InternalError': IDL.Text,
  });

  return IDL.Service({
    'authenticate_user': IDL.Func([], [IDL.Variant({ 'Ok': UserSession, 'Err': IDL.Text })], []),
    'renew_session': IDL.Func([], [IDL.Variant({ 'Ok': UserSession, 'Err': IDL.Text })], []),
    'logout': IDL.Func([], [IDL.Variant({ 'Ok': IDL.Text, 'Err': IDL.Text })], []),
    'create_session': IDL.Func([IDL.Opt(IDL.Text)], [IDL.Variant({ 'Ok': UserSession, 'Err': IDL.Text })], []),
    'update_last_active': IDL.Func([], [IDL.Variant({ 'Ok': IDL.Null, 'Err': IDL.Text })], []),
    'get_session': IDL.Func([], [IDL.Opt(UserSession)], ['query']),
    'is_authenticated': IDL.Func([], [IDL.Bool], ['query']),
    'validate_session': IDL.Func([IDL.Principal], [IDL.Variant({ 'Ok': UserSession, 'Err': AuthError })], ['query']),
    'set_user_role': IDL.Func([IDL.Principal, UserRole], [IDL.Variant({ 'Ok': IDL.Null, 'Err': IDL.Text })], []),
    'health_check': IDL.Func([], [IDL.Text], ['query']),
  });
};