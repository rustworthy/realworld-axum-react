import { FC, useState } from "react";

import { SrOnlyLabel } from "@/shared/styles/globalStyledComponents";
import { EyeNoneIcon, EyeOpenIcon } from "@radix-ui/react-icons";

import * as S from "../inputs.styles";
import { IInputProps } from "../inputs.types";

export const PasswordInput: FC<IInputProps> = ({ id, label, required, placeholder, error, field }) => {
  const [isPasswordRevealed, setIsPasswordRevealed] = useState<boolean>(false);

  return (
    <S.FormInputContainer>
      <SrOnlyLabel htmlFor={id}>{label}</SrOnlyLabel>
      <S.PasswordInput
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

      {error ? <S.FormInputError id={`${id}_error`}>{error}</S.FormInputError> : null}
    </S.FormInputContainer>
  );
};
