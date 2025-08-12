import { FC } from "react";

import { SrOnly } from "@/shared/ui/SrOnly";

import * as S from "../inputs.styles";
import { IInputProps } from "../inputs.types";

export const TextInput: FC<IInputProps> = ({ id, label, required, placeholder, error, field }) => {
  return (
    <S.FormInputContainer>
      <SrOnly>
        <label htmlFor={id}>Email</label>
      </SrOnly>
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
