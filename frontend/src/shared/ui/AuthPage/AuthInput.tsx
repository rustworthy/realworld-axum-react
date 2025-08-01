import { useState } from "react";

import { SrOnlyLabel } from "@/shared/styles/globalStyledComponents";
import { EyeNoneIcon, EyeOpenIcon } from "@radix-ui/react-icons";

import * as S from "./AuthInput.styles";

export const PasswordInput = ({
  id,
  label,
  error,
  required,
  ...field
}: {
  id: string;
  label: string;
  required: boolean;
  error: string;
}) => {
  const [isPasswordRevealed, setIsPasswordRevealed] = useState(false);
  return (
    <S.FormInputContainer>
      <SrOnlyLabel htmlFor={id}>Password</SrOnlyLabel>;
      <S.PasswordInput
        {...field}
        required={required}
        id={id}
        type={isPasswordRevealed ? "text" : "password"}
        placeholder="Password"
        autoComplete="off"
        aria-invalid={!!error}
        aria-errormessage={`${id}_error`}
      />
      <S.PasswordRevealToggle>
        {isPasswordRevealed ? (
          <EyeNoneIcon aria-role="button" aria-label="reveal password symbols" onClick={() => setIsPasswordRevealed(false)} />
        ) : (
          <EyeOpenIcon aria-role="button" aria-label="hide password symbols" onClick={() => setIsPasswordRevealed(true)} />
        )}
      </S.PasswordRevealToggle>
      {error ? <S.FormInputError id={`${id}_error`}>{error}</S.FormInputError> : null}
    </S.FormInputContainer>
  );
};
