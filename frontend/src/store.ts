import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";

import { api } from "./api";

export const store = configureStore({
  reducer: {
    [api.reducerPath]: api.reducer,
  },

  // this way we are enabling caching, invalidation, polling,
  // and other feature of `rtk-query`
  middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(api.middleware),
});

// required for `refetchOnFocus/refetchOnReconnect` functionality, see:
// https://redux-toolkit.js.org/rtk-query/api/setupListeners
setupListeners(store.dispatch);
