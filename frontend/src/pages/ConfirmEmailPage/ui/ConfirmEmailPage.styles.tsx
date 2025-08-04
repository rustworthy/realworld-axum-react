import styled from "@emotion/styled";

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

export const OTPInstruction = styled.p`
  padding-bottom: 1rem;
`;

export const OTPForm = styled.form`
  max-width: 25rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
`;
