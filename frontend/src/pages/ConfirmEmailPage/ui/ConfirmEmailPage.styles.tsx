import { MOBILE_WIDTH } from "@/shared/constants/styles.constants";
import styled from "@emotion/styled";

export const OTPForm = styled.form`
  width: 100%;
  padding: 0.5rem 0.75rem;
  display: flex;
  flex-direction: column;
`;

export const SubmissionSection = styled.div`
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 0.75rem 0.125rem;

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

export const OTPInstruction = styled.p`
  padding-bottom: 1rem;
`;
