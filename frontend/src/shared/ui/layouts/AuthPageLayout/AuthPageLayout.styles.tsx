import styled from "@emotion/styled";

export const PageWrapper = styled.div`
  padding-top: 3rem;
  width: 100%;
  height: 100%;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
`;

export const Title = styled.h1`
  margin-bottom: 0.5rem;
  font-family: inherit;
  font-weight: 500;
  font-size: 2.5rem;
  line-height: 1.1;
  color: inherit;
`;

export const PageInner = styled.div`
  width: 100%;
  max-width: 35rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
`;

export const ButtonContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: end;

  .SimpleButton {
    max-width: 7.5rem;
  }
`;
