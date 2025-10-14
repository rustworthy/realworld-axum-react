import styled from "@emotion/styled";

export const Wrapper = styled.div`
  width: 100dvw;
  min-height: 100dvh;
  background-color: ${({ theme }) => theme.mainLayout.backgroundColor};
  color: ${({ theme }) => theme.mainLayout.textColor};
  font-family: "Source Sans Pro", sans-serif;

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
`;

export const ChildrenContainer = styled.main`
  flex-grow: 1;
`;
