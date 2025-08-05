import { useDispatch, useSelector } from "react-redux";

import type { AppState } from "declarations";

import { ConfirmEmailApiArg, LoginApiArg, useConfirmEmailMutation, useLoginMutation } from "../api/generated";
import { type AuthSliceState, setLoggedOut } from "./slice";

export const useAuth = () => {
  const dispatch = useDispatch();
  const auth: AuthSliceState = useSelector((state: AppState) => state.auth);
  const [confirmEmailMutation, { isLoading: isConfirmEmailLoading }] = useConfirmEmailMutation();
  const [loginMutation, { isLoading: isLoginLoading }] = useLoginMutation();

  const confirmEmail = async (arg: ConfirmEmailApiArg) => {
    const result = await confirmEmailMutation(arg);
    return result;
  };

  const login = async (arg: LoginApiArg) => {
    const result = await loginMutation(arg);
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
