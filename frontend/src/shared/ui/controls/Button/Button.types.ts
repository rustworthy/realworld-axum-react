import { ReactNode } from "react";

export interface IButton {
  dataTestId: string;
  className?: string;
  children?: ReactNode;
  isDisabled?: boolean;
  onClick?: () => void;
}
