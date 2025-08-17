import { FC } from "react";

import { SrOnly } from "@/shared/ui/SrOnly";

import { FormInputContainer, FormInputError, FormInputErrorContainer } from "../inputs.styles";
import { IInputProps } from "../inputs.types";
import * as S from "./Textarea.styles";

export const Textarea: FC<IInputProps & { rows: number }> = ({ id, label, required, placeholder, error, field, rows }) => {
  return (
    <FormInputContainer>
      <SrOnly>
        <label htmlFor={id}>{label}</label>
      </SrOnly>
      <S.Textarea
        {...field}
        rows={rows}
        required={required}
        id={id}
        placeholder={placeholder ?? label}
        aria-invalid={!!error}
        aria-errormessage={`${id}_error`}
      />
      <FormInputErrorContainer>
        {error ? <FormInputError id={`${id}_error`}>{error}</FormInputError> : null}
      </FormInputErrorContainer>
    </FormInputContainer>
  );
};
