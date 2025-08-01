import { css } from "@emotion/react";
import styled from "@emotion/styled";

import {
  DESKTOP_CONTAINER_WIDTH,
  EXTRA_LARGE_WIDTH,
  LARGE_CONTAINER_WIDTH,
  LARGE_WIDTH,
  MOBILE_CONTAINER_WIDTH,
  TABLET_CONTAINER_WIDTH,
  TABLET_WIDTH,
} from "../constants/styles.constants";

export const LayoutContainer = styled.div`
  margin: 0 auto;
  height: 100%;
  width: 100%;
  max-width: ${DESKTOP_CONTAINER_WIDTH}px;

  @media (max-width: ${EXTRA_LARGE_WIDTH}px) {
    max-width: ${LARGE_CONTAINER_WIDTH}px;
  }
  @media (max-width: ${LARGE_WIDTH}px) {
    max-width: ${TABLET_CONTAINER_WIDTH}px;
  }
  @media (max-width: ${TABLET_WIDTH}px) {
    max-width: ${MOBILE_CONTAINER_WIDTH}px;
  }
`;

export const PositionAbsoluteHidden = css`
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  border: 0;
`;

export const SrOnlyLabel = styled.label`
  ${PositionAbsoluteHidden};
`;
