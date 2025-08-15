import { MOBILE_WIDTH } from "@/shared/constants/styles.constants";
import styled from "@emotion/styled";

export const SubmitButtonContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;

  .SimpleButton {
    max-width: 11.5rem;
  }

  @media (max-width: ${MOBILE_WIDTH}px) {
    flex-direction: column-reverse;
    align-items: center;
    gap: 1.75rem;

    .SimpleButton {
      max-width: 100%;
    }
  }
`;

export const LogoutSectionWrapper = styled.div`
  width: 100%;
  max-width: 35rem;
  padding: 0 0.75rem 1.5rem;
`

export const Separator = styled.hr`
  width: 100%;
  margin: 0.5rem 0 1rem;
  border-top: 2px solid;
  opacity: 0.1;
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

