// eslint-disable-next-line import/no-restricted-paths
import { store } from "@/app/providers/ReduxProvider/store";

export type AppState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
