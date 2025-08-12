import styled from "@emotion/styled";

export const Form = styled.form`
  width: 100%;
  display: flex;
  flex-direction: column;
`;

export const SubmitButtonContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;

  .SimpleButton {
    max-width: 11.5rem;
  }
`;

export const LogoutButtonContainer = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: start;

  .SimpleButton {
    height: 2.3rem;
    max-width: 11.55rem;
    font-size: 1rem;
    padding: 0.5rem 1rem;
    color: ${({ theme }) => theme.shared.button.danger.textColor};
    border-color: ${({ theme }) => theme.shared.button.danger.borderColor};
    background-color: ${({ theme }) => theme.shared.button.danger.backgroundColor};

    &:active,
    &:hover,
    &:focus {
      color: ${({ theme }) => theme.shared.button.danger.active.textColor};
      background-color: ${({ theme }) => theme.shared.button.danger.active.backgroundColor};
    }
  }
`;

export const Separator = styled.hr`
  width: 100%;
  margin: 1rem 0;
  border-top: 2px solid;
  opacity: 0.1;
`;
