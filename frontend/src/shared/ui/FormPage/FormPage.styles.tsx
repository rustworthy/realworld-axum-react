import styled from "@emotion/styled";

import { MOBILE_WIDTH, TABLET_CONTAINER_WIDTH } from "@/shared/constants/styles.constants";

import { LayoutContainer } from "../Container";

export const PageWrapper = styled(LayoutContainer)`
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

export const Form = styled.form<{ maxWidth?: string }>`
  width: 100%;
  max-width: ${(props) => props.maxWidth ?? `${TABLET_CONTAINER_WIDTH}px`};
  padding: 0.5rem 0.75rem;
  display: flex;
  flex-direction: column;
`;

export const FormSubmissionSection = styled.div`
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 0.75rem 0.125rem;
  gap: 1rem;

  .SimpleButton {
    max-width: 7.5rem;
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
