import styled from "@emotion/styled";

export const Button = styled.button`
  height: 54px;
  width: 100%;
  padding: 12px 24px;
  color: ${({ theme }) => theme.shared.button.textColor};
  background-color: ${({ theme }) => theme.shared.button.backgroundColor};
  border: 1px solid ${({ theme }) => theme.shared.button.borderColor};
  border-radius: 5px;

  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 8px;

  font-size: 20px;
  font-family: inherit;
  line-height: 24px;
  text-transform: none;
  cursor: pointer;
  user-select: none;

  &:active,
  &:hover {
    background-color: ${({ theme }) => theme.shared.button.backgroundColorActive};
  }
`;
