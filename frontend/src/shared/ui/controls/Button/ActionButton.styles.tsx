import styled from "@emotion/styled";

export const ActionButton = styled.button`
  background: transparent;
  border: 1px solid currentColor;
  color: inherit;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 4px;
  height: 2.3rem;
  width: 14rem;
  font-size: 1rem;
  padding: 0.5rem 1rem;

  &.btn-outline-primary {
    color: ${({ theme }) => theme.shared.button.primary.textColor};
    background-color: ${({ theme }) => theme.shared.button.primary.backgroundColor};
    border-color: ${({ theme }) => theme.shared.button.primary.borderColor};

    &:active:not([disabled]),
    &:hover:not([disabled]),
    &:focus:not([disabled]) {
      color: ${({ theme }) => theme.shared.button.primary.active.textColor};
      background-color: ${({ theme }) => theme.shared.button.primary.active.backgroundColor};
    }

    &:disabled {
      cursor: not-allowed;
    }
  }

  &.btn-outline-secondary {
    color: ${({ theme }) => theme.shared.button.secondary.textColor};
    background-color: ${({ theme }) => theme.shared.button.secondary.backgroundColor};
    border-color: ${({ theme }) => theme.shared.button.secondary.borderColor};

    &:active:not([disabled]),
    &:hover:not([disabled]),
    &:focus:not([disabled]) {
      color: ${({ theme }) => theme.shared.button.secondary.active.textColor};
      background-color: ${({ theme }) => theme.shared.button.secondary.active.backgroundColor};
    }

    &:disabled {
      cursor: not-allowed;
    }
  }

  &.btn-outline-danger {
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

  &.compact {
    width: 9.5rem;
  }

  &.fit {
    width: fit-content;
  }
`;
