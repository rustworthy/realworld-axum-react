import { SrOnlyLabel } from "@/shared/styles/globalStyledComponents";

import * as S from "./AuthInput.styles";
import { InputProps } from "./types";

export const TextInput = ({ id, label, required, placeholder, error, ...rest }: InputProps) => {
  return (
    <S.FormInputContainer>
      <SrOnlyLabel htmlFor="signup_email">Email</SrOnlyLabel>
      <S.FormInput
        {...rest}
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
