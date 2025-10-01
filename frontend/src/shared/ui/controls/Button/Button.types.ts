import { ReactNode } from "react";

export interface IButtonProps {
  dataTestId?: string;
  className?: string;
  children?: ReactNode;
  isDisabled?: boolean;
  onClick?: () => void;
}
