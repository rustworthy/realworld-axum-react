import { createSlice } from "@reduxjs/toolkit";

import { UserPayloadUser } from "@/shared/api/generated";

import { AuthSliceState } from "./authSlice.schema";

export const authSliceLoggedOutState: AuthSliceState = {
  user: null,
  isAuthenticated: false,
};

const authSlice = createSlice({
  name: "auth",
  initialState: authSliceLoggedOutState,
  reducers: {
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
});

export const { setLoggedIn, setLoggedOut, restoreSnapshot } = authSlice.actions;

export const authReducer = authSlice.reducer;
