import { FC, ReactNode } from "react";
import { Provider as StateProvider } from "react-redux";

import { store } from "./store";

interface IReduxProvider {
  children: ReactNode;
}

export const ReduxProvider: FC<IReduxProvider> = (props) => {
  return <StateProvider store={store}> {props.children} </StateProvider>;
};
