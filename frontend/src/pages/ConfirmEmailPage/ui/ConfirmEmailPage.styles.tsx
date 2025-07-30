import { MuiOtpInput } from "mui-one-time-password-input";
import styled from "styled-components";

export const OtpInput = styled(MuiOtpInput)`
  display: flex;
  gap: 0.5rem !important;
  align-item: space-between;
  max-width: 35rem;
  margin-inline: auto;
`;

export const ButtonContainer = styled.div`
  width: 100%;
  max-width: 600px;
  display: flex;
  flex-direction: column;
  align-items: end;

  .SimpleButton {
    max-width: 7.5rem;
  }
`;
