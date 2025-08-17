import { FC } from "react";

import { SrOnly } from "@/shared/ui/SrOnly";

import * as S from "../inputs.styles";
import { IInputProps } from "../inputs.types";

export const TextInput: FC<IInputProps> = ({ id, label, required, placeholder, autoComplete, error, field }) => {
  return (
    <S.FormInputContainer>
      <SrOnly>
        <label htmlFor={id}>{label}</label>
      </SrOnly>
      <S.FormInput
        {...field}
        required={required}
        id={id}
        type="text"
        placeholder={placeholder ?? label}
        autoComplete={autoComplete ?? "off"}
        aria-invalid={!!error}
        aria-errormessage={`${id}_error`}
      />

      <S.FormInputErrorContainer>
        {error ?
          <S.FormInputError id={`${id}_error`}>{error}</S.FormInputError>
          : null
        }
      </S.FormInputErrorContainer>
    </S.FormInputContainer>
  );
};
