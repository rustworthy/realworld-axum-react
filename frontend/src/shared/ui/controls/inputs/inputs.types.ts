import { UseControllerReturn } from "react-hook-form";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type RHFFieldProps = Partial<UseControllerReturn<any>["field"]>;

export interface IInputProps {
  id: string;
  label: string;
  required: boolean;
  placeholder?: string;
  autoComplete?: string;
  error?: string;
  field?: RHFFieldProps;
}

export interface IOTPInputProps extends IInputProps {
  length: number;
}
