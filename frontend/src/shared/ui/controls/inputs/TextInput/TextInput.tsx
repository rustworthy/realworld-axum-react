import { FC } from "react";

import { SrOnlyLabel } from "@/shared/styles/globalStyledComponents";

import * as S from "../inputs.styles";
import { IInputProps } from "../inputs.types";

export const TextInput: FC<IInputProps> = ({ id, label, required, placeholder, error, field }) => {
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
