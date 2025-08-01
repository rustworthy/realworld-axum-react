import {  UseControllerReturn } from "react-hook-form";

type RHFFieldProps = Partial<UseControllerReturn<any>['field']>;

export interface IInputProps {
  id: string;
  label: string;
  required: boolean;
  placeholder?: string;
  error?: string;
  field?: RHFFieldProps;
}
