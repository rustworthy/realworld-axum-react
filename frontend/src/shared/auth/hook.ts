import { useDispatch, useSelector } from "react-redux";

import type { AppState } from "declarations";

import type { AuthSliceState } from "./reducer";

export const useAuth = () => {
  const _dispatch = useDispatch();
  const _auth: AuthSliceState = useSelector((state: AppState) => state.auth);
};
