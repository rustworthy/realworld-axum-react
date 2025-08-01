export interface IInputProps {
  id: string;
  label: string;
  required: boolean;
  placeholder?: string;
  error?: string;
  [K: string]: unknown;
}

export interface IOTPInputProps extends IInputProps {
  length: number;
}
