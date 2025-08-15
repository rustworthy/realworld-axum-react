import {
  FieldErrors,
  FieldValues,
  UseControllerReturn,
  UseFormClearErrors,
  UseFormSetError,
  UseFormSetValue,
} from "react-hook-form";

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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export interface ITurnstileInputProps<T extends FieldValues = any> {
  name: string;
  setValue: UseFormSetValue<T>;
  setError: UseFormSetError<T>;
  clearErrors: UseFormClearErrors<T>;
  fieldErrors: FieldErrors<T>;
}
