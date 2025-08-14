import styled from "@emotion/styled";

export const SignInForm = styled.form`
  width: 100%;
  padding: 0.5rem 0.75rem;
  display: flex;
  flex-direction: column;
`;

export const SignInButtonContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;

  .SimpleButton {
    max-width: 7.5rem;
  }
`;

export const SignUpLink = styled.a`
  color: ${({ theme }) => theme.page.signin.signUpLink.textColor};
`;
