import styled from "@emotion/styled";
import { Link } from "react-router";

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

export const AuthorImage = styled.img`
  width: 32px;
  height: 32px;
  object-fit: cover;
  object-position: top center;
  border-radius: 50%;
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
