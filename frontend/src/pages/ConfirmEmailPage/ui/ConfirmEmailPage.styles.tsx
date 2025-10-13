import styled from "@emotion/styled";

import { MOBILE_CONTAINER_WIDTH } from "@/shared/constants/styles.constants";
import { FormPage } from "@/shared/ui/FormPage";

export const OTPInstruction = styled.p`
  padding-bottom: 1rem;
`;

export const OTPForm = styled(FormPage.Form)`
  max-width: ${MOBILE_CONTAINER_WIDTH}px;
`;
