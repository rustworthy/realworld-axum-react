import styled from "@emotion/styled";

export const TagList = styled.ul`
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
`;

export const Tag = styled.li<{ $interactive?: boolean }>`
  background: ${({ theme }) => theme.shared.tag.backgroundColor};
  color: ${({ theme }) => theme.shared.tag.textColor};
  padding: 0.125rem 0.75rem;
  border-radius: 10rem;
  white-space: nowrap;
  min-width: 2rem;
  line-height: 1.4rem;
  font-size: 0.9rem;

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
