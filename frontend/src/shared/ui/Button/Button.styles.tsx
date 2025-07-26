import styled from "styled-components";

export const Button = styled.button`
  height: 48px;
  width: 100%;
  padding: 12px 24px 12px 24px;
  color: ${({ theme }) => theme.shared.button.c02};
  background-color: ${({ theme }) => theme.shared.button.c01};
  border: 1px solid ${({ theme }) => theme.shared.button.c01};
  border-radius: 20px;

  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8px;

  font-size: 16px;
  font-weight: 700;
  line-height: 24px;
  text-transform: none;
  cursor: pointer;

  &:hover {
    background-color: ${({ theme }) => theme.shared.button.c03};
    box-shadow: 0 4px 24px 0 #4a55073d;
  }
`;
