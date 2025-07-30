import styled from "styled-components";

export const PageWrapper = styled.div`
  padding-top: 3rem;
  width: 100%;
  height: 100%;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
`;

export const Title = styled.h1`
  margin-bottom: 0.5rem;
  font-family: inherit;
  font-weight: 500;
  font-size: 2.5rem;
  line-height: 1.1;
  color: inherit;
`;

export const SubTitle = styled.a`
  color: ${(props) => props.theme.page.signup.colorAccent};
`;

export const SignUpForm = styled.form`
  width: 100%;
  max-width: 540px;
  padding: 0.5rem 0.75rem;
  display: flex;
  flex-direction: column;
`;

export const FormInputContainer = styled.div`
  position: relative;
`;

export const FormInput = styled.input`
  display: block;
  width: 100%;
  line-height: 1.25;
  color: ${(props) => props.theme.shared.input.textColor};
  background-color: ${(props) => props.theme.shared.input.backgroundColor};
  background-clip: padding-box;
  border: 1px solid ${(props) => props.theme.shared.input.borderColor};
  padding: 0.75rem 1.5rem;
  font-size: 1.25rem;
  border-radius: 0.3rem;
  margin-bottom: 1.2rem;

  &:focus {
    border-color: ${(props) => props.theme.shared.input.backgroundColorFocused};
    outline: none;
  }
`;

export const FormInputError = styled.span`
  position: absolute;
  bottom: 0.1rem;
  left: 0;
  color: ${(props) => props.theme.shared.input.errorColor};
`;

export const SignUpButtonContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;

  .SimpleButton {
    max-width: 7.5rem;
  }
`;
