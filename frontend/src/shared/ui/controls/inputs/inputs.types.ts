import {
  FieldErrors,
  FieldValues,
  UseControllerReturn,
  UseFormClearErrors,
  UseFormSetError,
  UseFormSetValue,
} from "react-hook-form";

import { ANY_TODO } from "@/shared/types/common.types";

type RHFFieldProps = Partial<UseControllerReturn<ANY_TODO>["field"]>;

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

export interface ICaptchaInputProps<T extends FieldValues = ANY_TODO> {
  name: string;
  setValue: UseFormSetValue<T>;
  setError: UseFormSetError<T>;
  clearErrors: UseFormClearErrors<T>;
  fieldErrors: FieldErrors<T>;
}
