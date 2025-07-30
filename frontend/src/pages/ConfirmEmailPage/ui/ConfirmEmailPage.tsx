import { useState } from "react";

import { AuthPage } from "@/shared/ui/AuthPage/AuthPage";
import { Button } from "@/shared/ui/Button";

import * as S from "./ConfirmEmailPage.styles";

export const ConfirmEmailPage = () => {
  const [otp, setOtp] = useState("");
  return (
    <AuthPage title="Let's confirm your email">
      <p>Please insert a one-time code we've sent to you via email.</p>
      <S.OtpInput length={8} value={otp} onChange={setOtp} />
      <S.ButtonContainer>
        <Button dataTestId="confirm_email_button" isDisabled={false}>
          Submit
        </Button>
      </S.ButtonContainer>
    </AuthPage>
  );
};
