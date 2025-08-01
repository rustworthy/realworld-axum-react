import styled from "@emotion/styled";

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
  padding: 1rem 1.5rem;
  font-size: 1.25rem;
  border-radius: 0.3rem;
  margin-bottom: 1.2rem;

  &:focus {
    border-color: ${(props) => props.theme.shared.input.backgroundColorFocused};
    outline: none;
  }
`;

export const PasswordInput = styled(FormInput)`
  padding-right: 3rem;
`;

export const PasswordRevealToggle = styled.div`
  right: 1rem;
  top: 1.125rem;
  position: absolute;
  width: fit-content;
  height: fit-content;

  svg {
    width: 20px;
    height: 20px;
    color: ${(props) => props.theme.shared.input.textColor};
  }
`;

export const FormInputError = styled.span`
  position: absolute;
  bottom: 0.1rem;
  left: 0;
  color: ${(props) => props.theme.shared.input.errorColor};
`;
