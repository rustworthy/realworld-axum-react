import {
  FieldErrors,
  FieldValues,
  UseControllerReturn,
  UseFormClearErrors,
  UseFormSetError,
  UseFormSetValue,
} from "react-hook-form";

import { ANY_TODO } from "@/shared/types/common.types";
import { MDEditorProps } from "@uiw/react-md-editor";

type RHFFieldProps = Partial<UseControllerReturn<ANY_TODO>["field"]>;

export interface IInputProps {
  id: string;
  label: string;
  required: boolean;
  placeholder?: string;
  autoComplete?: string;
  autoFocus?: boolean;
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

export interface IEditorInputProps extends IInputProps {
  name: string;
  value: MDEditorProps["value"];
  onChange?: MDEditorProps["onChange"];
  maxLength?: number;
}
