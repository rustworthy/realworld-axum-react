import styled from "@emotion/styled";

export const SignUpForm = styled.form`
  width: 100%;
  padding: 0.5rem 0.75rem;
  display: flex;
  flex-direction: column;
`;

export const SignUpSubmissionSection = styled.div`
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 0.75rem 0.125rem;

  .SimpleButton {
    max-width: 7.5rem;
  }
`;

export const SignInLink = styled.a`
  color: ${({ theme }) => theme.page.signup.signInLink.textColor};
`;
