import { useDispatch, useSelector } from "react-redux";

import { SerializedError } from "@reduxjs/toolkit";
import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { ConfirmEmailApiArg, LoginApiArg, UserPayloadUser, useConfirmEmailMutation, useLoginMutation } from "../api/generated";
import { AppState } from "../providers/ReduxProvider/store";
import { type AuthSliceState, setLoggedIn, setLoggedOut } from "./slice";

export type MaybeUserPayload =
  | { data: UserPayloadUser; error: undefined }
  | { data: undefined; error: FetchBaseQueryError | SerializedError };
export type ConfirmEmailFnType = (arg: ConfirmEmailApiArg) => Promise<MaybeUserPayload>;
export type LoginFnType = (arg: LoginApiArg) => Promise<MaybeUserPayload>;

export type UseAuthHookReturnType = {
  user: AuthSliceState["user"];
  isAuthenticated: boolean;
  confirmEmail: ConfirmEmailFnType;
  isConfirmEmailLoading: boolean;
  login: LoginFnType;
  isLoginLoading: boolean;
  logout: () => void;
};

export const useAuth = (): UseAuthHookReturnType => {
  const dispatch = useDispatch();
  const auth: AuthSliceState = useSelector((state: AppState) => state.auth);
  const [confirmEmailMutation, { isLoading: isConfirmEmailLoading }] = useConfirmEmailMutation();
  const [loginMutation, { isLoading: isLoginLoading }] = useLoginMutation();

  const confirmEmail = async (arg: ConfirmEmailApiArg): Promise<MaybeUserPayload> => {
    const result = await confirmEmailMutation(arg);
    if (result.data) dispatch(setLoggedIn(result.data));
    return result;
  };

  const login = async (arg: LoginApiArg): Promise<MaybeUserPayload> => {
    const result = await loginMutation(arg);
    if (result.data) dispatch(setLoggedIn(result.data));
    return result;
  };

  const logout = () => {
    dispatch(setLoggedOut());
  };

  return {
    user: auth.user,
    isAuthenticated: auth.isAuthenticated,

    confirmEmail,
    isConfirmEmailLoading,

    login,
    isLoginLoading,
    logout,
  };
};
