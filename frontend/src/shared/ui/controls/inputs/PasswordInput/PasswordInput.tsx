import { FC, useState } from "react";

import { SrOnly } from "@/shared/ui/SrOnly";
import { EyeNoneIcon, EyeOpenIcon } from "@radix-ui/react-icons";

import * as S from "../inputs.styles";
import { IInputProps } from "../inputs.types";

export const PasswordInput: FC<IInputProps> = ({ id, label, required, placeholder, error, field }) => {
  const [isPasswordRevealed, setIsPasswordRevealed] = useState<boolean>(false);

  return (
    <S.FormInputContainer>
      <SrOnly>
        <label htmlFor={id}>{label}</label>
      </SrOnly>
      <S.FormInput
        $padding="1rem 3rem 1rem 1.5rem"
        {...field}
        required={required}
        id={id}
        type={isPasswordRevealed ? "text" : "password"}
        placeholder={placeholder ?? label}
        autoComplete="off"
        aria-invalid={!!error}
        aria-errormessage={`${id}_error`}
      />

      <S.PasswordRevealToggle>
        {isPasswordRevealed ? (
          <EyeNoneIcon role="button" aria-label="reveal password symbols" onClick={() => setIsPasswordRevealed(false)} />
        ) : (
          <EyeOpenIcon role="button" aria-label="hide password symbols" onClick={() => setIsPasswordRevealed(true)} />
        )}
      </S.PasswordRevealToggle>

      <S.FormInputErrorContainer>
        {error ? <S.FormInputError id={`${id}_error`}>{error}</S.FormInputError> : null}
      </S.FormInputErrorContainer>
    </S.FormInputContainer>
  );
};
