import { SrOnlyLabel } from "@/shared/styles/globalStyledComponents";
import { Button } from "@/shared/ui/Button";

import * as S from "./SignUpPage.styles";

export const SignUpPage = () => {
  return (
    <S.PageWrapper>
      <S.Title>Sign up</S.Title>
      <S.SubTitle href="/signin">Have an account?</S.SubTitle>
      <S.SignUpForm>
        <SrOnlyLabel htmlFor="signup_username">Username</SrOnlyLabel>
        <S.FormInput id="signup_username" type="text" placeholder="Username" autoComplete="off" />

        <SrOnlyLabel htmlFor="signup_email">Email</SrOnlyLabel>
        <S.FormInput id="signup_email" type="text" placeholder="Email" autoComplete="off" />

        <SrOnlyLabel htmlFor="signup_password">Password</SrOnlyLabel>
        <S.FormInput id="signup_password" type="password" placeholder="Password" autoComplete="off" />

        <S.SignUpButtonContainer>
          <Button dataTestId="signup_submit_button" isDisabled={false} onClick={() => {}}>
            Sign up
          </Button>
        </S.SignUpButtonContainer>
      </S.SignUpForm>
    </S.PageWrapper>
  );
};
