import styled from "@emotion/styled";

import { MOBILE_WIDTH } from "@/shared/constants/styles.constants";

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
