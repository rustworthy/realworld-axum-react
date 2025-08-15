import styled from "@emotion/styled";

import { MOBILE_WIDTH } from "@/shared/constants/styles.constants";

export const SlotGroupsWrapper = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  padding-bottom: 1.5rem;
`;

export const SlotGroup = styled.div`
  display: flex;
  gap: 0.25rem;
`;

export const Slot = styled.div<{ isActive?: boolean }>`
  position: relative;
  width: 2.5rem;
  height: 3.5rem;
  font-size: 1.25rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  color: ${(props) => props.theme.shared.input.textColor};
  background-color: ${(props) => props.theme.shared.input.backgroundColor};
  border: 1px solid ${(props) => props.theme.shared.input.borderColor};

  &:first-of-type {
    border-top-left-radius: 0.3rem;
    border-bottom-left-radius: 0.3rem;
  }
  &:last-of-type {
    border-top-right-radius: 0.3rem;
    border-bottom-right-radius: 0.3rem;
  }

  @media (max-width: ${MOBILE_WIDTH}px) {
    width: 2.25rem;
  }
`;

export const CharContainer = styled.div`
  .group:has(input[data-input-otp-placeholder-shown]) & {
    opacity: 0.2;
  }
`;

export const FakeCaretContainer = styled.div`
  position: absolute;
  pointer-events: none;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: caret-blink 1.2s ease-out infinite;

  @keyframes caret-blink {
    0%,
    70%,
    100% {
      opacity: 1;
    }
    20%,
    50% {
      opacity: 0;
    }
  }
`;

export const CaretLine = styled.div`
  width: 0.1rem;
  height: 2rem;
  background-color: ${({ theme }) => theme.shared.input.textColor};
`;

export const DashLine = styled.div`
  width: 0.75rem;
  height: 0.126rem;
  border-radius: 9999px;
  background-color: ${({ theme }) => theme.shared.input.otp.separator.backgroundColor};
`;
