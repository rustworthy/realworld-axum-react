import styled from "@emotion/styled";

export const CaptchaInputContainer = styled.div`
  max-width: 350px;
`;

export const CaptchaWidgetContainer = styled.div`
  width: 300px;
  height: 65px;
`;

export const CaptchaError = styled.span`
  color: ${(props) => props.theme.shared.input.errorColor};
`;
