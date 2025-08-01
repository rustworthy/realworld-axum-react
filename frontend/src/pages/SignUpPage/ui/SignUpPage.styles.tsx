import styled from "@emotion/styled";

export const SignInLink = styled.a`
  color: ${(props) => props.theme.page.signup.colorAccent};
`;

export const SignUpForm = styled.form`
  width: 100%;
  padding: 0.5rem 0.75rem;
  display: flex;
  flex-direction: column;
`;

export const SignUpButtonContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;

  .SimpleButton {
    max-width: 7.5rem;
  }
`;
