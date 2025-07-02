import { ReactNode } from "react";

export interface IButton {
  dataTestId: string;
  children?: ReactNode;
  isDisabled?: boolean;
  onClick: () => void;
}
