import { AuthPageLayout } from "@/shared/ui/layouts";
import { Button } from "@/shared/ui/controls/Button";

import * as S from "./ConfirmEmailPage.styles";

export const ConfirmEmailPage = () => {
  return (
    <AuthPageLayout title="Let's confirm your email">
      <p>Please insert a one-time code we've sent to you via email.</p>

      <S.ButtonContainer>
        <Button dataTestId="confirm_email_button" isDisabled={false}>
          Submit
        </Button>
      </S.ButtonContainer>
    </AuthPageLayout>
  );
};
