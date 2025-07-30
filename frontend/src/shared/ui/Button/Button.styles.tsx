import styled from "styled-components";

export const Button = styled.button`
  height: 50px;
  width: 100%;
  padding: 12px 24px;
  color: ${({ theme }) => theme.shared.button.c02};
  background-color: ${({ theme }) => theme.shared.button.c03};
  border: 1px solid ${({ theme }) => theme.shared.button.c03};
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

  &:active,
  &:hover {
    background-color: ${({ theme }) => theme.shared.button.c04};
  }
`;
