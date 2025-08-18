import { createSlice } from "@reduxjs/toolkit";

import { UserPayloadUser } from "@/shared/api/generated";

import { AuthSliceState } from "./authSlice.schema";

export const authSliceLoggedOutState: AuthSliceState = {
  loadingCount: 0,
  user: null,
  isAuthenticated: false,
};

const authSlice = createSlice({
  name: "auth",
  initialState: authSliceLoggedOutState,
  reducers: {
    startLoading: (state) => {
      state.loadingCount += 1;
    },
    stopLoading: (state) => {
      state.loadingCount = Math.max(state.loadingCount - 1, 0);
    },
    setLoggedIn: (state, action: { payload: UserPayloadUser }) => {
      state.isAuthenticated = true;
      state.user = action.payload.user;
    },
    setLoggedOut: (state) => {
      state.isAuthenticated = authSliceLoggedOutState.isAuthenticated;
      state.user = authSliceLoggedOutState.user;
    },
    restoreSnapshot: (state, action: { payload: AuthSliceState }) => {
      state.isAuthenticated = action.payload.isAuthenticated;
      state.user = action.payload.user;
    },
  },
  selectors: {
    isLoading: (state) => state.loadingCount > 0,
  },
});

export const { startLoading, stopLoading, setLoggedIn, setLoggedOut, restoreSnapshot } = authSlice.actions;
export const { isLoading } = authSlice.selectors;

export const authReducer = authSlice.reducer;
