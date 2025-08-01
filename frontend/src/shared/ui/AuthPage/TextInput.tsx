import { SrOnlyLabel } from "@/shared/styles/globalStyledComponents";

import * as S from "./AuthInput.styles";

export const TextInput = ({
  id,
  label,
  required,
  placeholder,
  error,
  ...field
}: {
  id: string;
  label: string;
  required: boolean;
  placeholder?: string;
  error?: string;
}) => {
  return (
    <S.FormInputContainer>
      <SrOnlyLabel htmlFor="signup_email">Email</SrOnlyLabel>
      <S.FormInput
        {...field}
        required={required}
        id={id}
        type="text"
        placeholder={placeholder ?? label}
        autoComplete="off"
        aria-invalid={!!error}
        aria-errormessage={`${id}_error`}
      />
      {error ? <S.FormInputError id={`${id}_error`}>{error}</S.FormInputError> : null}
    </S.FormInputContainer>
  );
};
