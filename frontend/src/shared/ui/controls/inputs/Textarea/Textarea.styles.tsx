import styled from "@emotion/styled";

export const Textarea = styled.textarea`
  resize: none;
  width: 100%;
  line-height: 1.25;
  color: ${(props) => props.theme.shared.input.textColor};
  background-color: ${(props) => props.theme.shared.input.backgroundColor};
  background-clip: padding-box;
  padding: 1rem 1.5rem;
  font-size: 1.325rem;
  border-radius: 0.3rem;
  margin-bottom: 1.2rem;

  border-width: 1px;
  border-color: ${(props) => props.theme.shared.input.borderColor};
  font-family: inherit;

  &:focus {
    outline: none;
    border-color: ${(props) => props.theme.shared.input.borderColorFocused};
  }
  &::placeholder {
    opacity: 0.8;
  }

  &::-webkit-scrollbar {
    width: 1rem;
  }
  &::-webkit-scrollbar-track {
    border-top-right-radius: 0.3rem;
    border-bottom-right-radius: 0.3rem;
    -webkit-box-shadow: inset 0px 0px 0.5rem 0.5rem ${(props) => props.theme.shared.input.backgroundColor};
  }
  &::-webkit-scrollbar-thumb {
  }
`;
