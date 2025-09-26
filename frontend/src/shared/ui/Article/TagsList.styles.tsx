import styled from "@emotion/styled";

export const TagList = styled.ul`
  display: flex;
  gap: 0.5rem;
`;

export const Tag = styled.li<{ $interactive?: boolean }>`
  background: ${({ theme }) => theme.shared.tag.backgroundColor};
  color: ${({ theme }) => theme.shared.tag.textColor};
  padding: 0.125rem 0.5rem;
  border-radius: 10rem;
  white-space: nowrap;
  min-width: 1.75rem;
  font-size: 0.75rem;

  &:hover {
    cursor: ${({ $interactive }) => ($interactive ? "pointer" : "default")};
    background: ${({ $interactive, theme }) => ($interactive ? theme.shared.tag.active.backgroundColor : "")};
  }

  &.outline {
    background: transparent;
    border: 1px solid ${({ theme }) => theme.shared.tag.outline.textColor};
    color: ${({ theme }) => theme.shared.tag.outline.textColor};
  }
`;
