import { createSlice } from "@reduxjs/toolkit";

import { UserPayloadUser } from "@/shared/api/generated";
import z from "zod";

export const authSliceSchema = z.object({
  isAuthenticated: z.boolean(),
  user: z
    .object({
      username: z.string().nonempty(),
      email: z.email(),
      image: z.string().nullable(),
      bio: z.string(),
      token: z.jwt(),
    })
    .nullable(),
});

export type AuthSliceState = z.infer<typeof authSliceSchema>;

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
