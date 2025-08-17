import styled from "@emotion/styled";

export const FormInputContainer = styled.div`
  position: relative;
`;

/**
 *
 * WibKit autofill properties are set to prevent the browser from overriding
 * our styles (those default styles look especially ugly in dark mode)
 *
 * @see https://stackoverflow.com/a/14205976
 */
export const FormInput = styled.input<{ $withRightAddon?: boolean }>`
  display: block;
  width: 100%;
  line-height: 1.25;
  color: ${(props) => props.theme.shared.input.textColor};
  background-color: ${(props) => props.theme.shared.input.backgroundColor};
  background-clip: padding-box;
  border: 1px solid ${(props) => props.theme.shared.input.borderColor};
  padding-block: 1rem;
  padding-left: 1rem;
  padding-right: ${(props) => (props.$withRightAddon ? "3rem" : "1rem")};
  font-size: 1.25rem;
  border-radius: 0.3rem;

  &:focus {
    border-color: ${(props) => props.theme.shared.input.borderColorFocused};
    outline: none;
  }
  &::placeholder {
    opacity: 0.8;
  }
  &:is(
      :autofill,
      :-webkit-autofill,
      :-webkit-autofill:hover,
      :-webkit-autofill:active,
      :-webkit-autofill:focus
  ) {
    -webkit-background-clip: text;
    -webkit-text-fill-color: ${(props) => props.theme.shared.input.textColor};
    transition: background-color 5000000s ease-in-out 0s;
    transition: color 5000000s ease-in-out 0s;
    box-shadow: inset 0 0 2rem 2rem ${(props) => props.theme.shared.input.backgroundColor};
  }
}
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

export const FormInputErrorContainer = styled.div`
  min-height: 1.5rem;
`;
export const FormInputError = styled.span`
  line-height: 1;
  color: ${(props) => props.theme.shared.input.errorColor};
`;
