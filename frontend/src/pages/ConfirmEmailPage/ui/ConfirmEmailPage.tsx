import { AuthPage } from "@/shared/ui/AuthPage/AuthPage";
import { Button } from "@/shared/ui/Button";

import * as S from "./ConfirmEmailPage.styles";

export const ConfirmEmailPage = () => {
  return (
    <AuthPage title="Let's confirm your email">
      <p>Please insert a one-time code we've sent to you via email.</p>
      <S.ButtonContainer>
        <Button dataTestId="confirm_email_button" isDisabled={false}>
          Submit
        </Button>
      </S.ButtonContainer>
    </AuthPage>
  );
};
