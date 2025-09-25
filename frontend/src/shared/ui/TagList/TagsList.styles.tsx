import styled from "@emotion/styled";

export const TagList = styled.ul`
  list-style: none;
  padding: 0;
  margin: 24px 0;
  display: flex;
  gap: 8px;
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
`;
