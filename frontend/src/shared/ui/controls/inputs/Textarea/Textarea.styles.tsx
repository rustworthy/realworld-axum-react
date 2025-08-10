import styled from "@emotion/styled";

export const Textarea = styled.textarea`
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
    border-color: ${(props) => props.theme.shared.input.borderColorFocused};
    outline: none;
  }
  &::placeholder {
    opacity: 0.8;
  }
`;

