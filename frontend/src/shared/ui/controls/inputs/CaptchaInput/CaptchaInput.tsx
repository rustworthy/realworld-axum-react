import { FC, useCallback } from "react";

import { Turnstile } from "@marsidev/react-turnstile";
import { useTernaryDarkMode } from "usehooks-ts";

import { FormInputError } from "../inputs.styles";
import { ICaptchaInputProps } from "../inputs.types";
import * as S from "./CaptchaInput.styles";

export const CaptchaInput: FC<ICaptchaInputProps> = ({ name, setValue, setError, clearErrors, fieldErrors }) => {
  const { ternaryDarkMode } = useTernaryDarkMode();
  const theme = ternaryDarkMode === "system" ? "auto" : ternaryDarkMode === "dark" ? "dark" : "light";

  const onSuccess = useCallback((token: string) => {
    clearErrors([name]);
    setValue(name, token);
  }, [clearErrors, setValue]);

  const onError = useCallback(() => {
    setError(name, {
      message:
        "Anti-bot check failed, but no worries: try re-submitting in a few seconds. Please contact us if the issue persists. Apologies for this inconvenience.",
    });
    setValue(name, "");
  }, [setError, setValue]);

  return (
    <S.CaptchaInputContainer>
      <S.CaptchaWidgetContainer>
        <Turnstile siteKey="1x00000000000000000000AA" onSuccess={onSuccess} onError={onError} options={{ theme }} />
      </S.CaptchaWidgetContainer>
      {fieldErrors[name] ? <S.CaptchaError>{fieldErrors[name]!.message as string}</S.CaptchaError> : null}
    </S.CaptchaInputContainer>
  );
};
