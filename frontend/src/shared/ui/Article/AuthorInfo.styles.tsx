import { Link } from "react-router";

import styled from "@emotion/styled";

export const AuthorInfo = styled.div`
  display: flex;
  align-items: center;
  gap: 0.5rem;
`;

export const AuthorInfoNameBlock = styled.div`
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  line-height: 1.2;
`;

export const AuthorName = styled(Link)`
  color: inherit;
  text-decoration: none;
  font-weight: 500;

  &:hover {
    text-decoration: underline;
  }
`;

export const ArticleDate = styled.span`
  opacity: 0.7;
  font-size: 14px;
`;
