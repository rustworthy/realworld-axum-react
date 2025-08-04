import { UseControllerReturn } from "react-hook-form";

import { ANY_TODO } from "@/shared/types/theme.types";

type RHFFieldProps = Partial<UseControllerReturn<ANY_TODO>["field"]>;

export interface IInputProps {
  id: string;
  label: string;
  required: boolean;
  placeholder?: string;
  error?: string;
  field?: RHFFieldProps;
}

export interface IOTPInputProps extends IInputProps {
  length: number;
}
