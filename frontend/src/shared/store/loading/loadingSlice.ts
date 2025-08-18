import { createSlice } from "@reduxjs/toolkit";

import { LoadingSliceState } from "./loadingSlice.schema";

export const authSliceLoggedOutState: LoadingSliceState = {
  loadingCount: 0,
};

const loadingSlice = createSlice({
  name: "loading",
  initialState: authSliceLoggedOutState,
  reducers: {
    startLoading: (state) => {
      state.loadingCount += 1;
    },
    stopLoading: (state) => {
      state.loadingCount = Math.max(state.loadingCount - 1, 0);
    },
  },
  selectors: {
    isLoading: (state) => state.loadingCount > 0,
  },
});

export const { startLoading, stopLoading } = loadingSlice.actions;
export const { isLoading } = loadingSlice.selectors;

export const loadingReducer = loadingSlice.reducer;
