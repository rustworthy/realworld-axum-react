import { createSlice } from "@reduxjs/toolkit";

import { UserPayloadUser } from "@/shared/api/generated";

export interface AuthSliceState {
  user: UserPayloadUser["user"] | null;
  isAuthenticated: boolean;
}

const authSlice = createSlice({
  name: "auth",
  initialState: {
    user: null,
    isAuthenticated: false,
  } as AuthSliceState,
  reducers: {
    setLoggedIn: (state, action: { payload: UserPayloadUser }) => {
      state.isAuthenticated = true;
      state.user = action.payload.user;
    },
    setLoggedOut: (state) => {
      state.isAuthenticated = false;
      state.user = null;
    },
  },
});

export const { setLoggedIn, setLoggedOut } = authSlice.actions;

export const authReducer = authSlice.reducer;
