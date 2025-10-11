import styled from "@emotion/styled";

import { LayoutContainer } from "@/shared/ui/Container";

export const PageWrapper = styled.div`
  width: 100%;
  height: 100%;
  background-color: ${({ theme }) => theme.page.article.content.backgroundColor};
  color: ${({ theme }) => theme.page.article.content.textColor};
`;

export const Banner = styled.div`
  background: ${({ theme }) => theme.page.article.banner.backgroundColor};
  color: ${({ theme }) => theme.page.article.banner.textColor};
  padding: 2rem 0.5rem;
`;

export const BannerContainer = styled(LayoutContainer)`
  max-width: 1000px;
  display: flex;
  flex-direction: column;
  gap: 16px;
`;

export const MainContent = styled(LayoutContainer)`
  max-width: 1000px;
  padding: 2rem 0.5rem;
`;

export const ArticleTitle = styled.h1`
  font-weight: 600;
  font-size: 45px;
  margin: 0;
  color: inherit;
`;

export const ArticleContent = styled.div`
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  color: ${({ theme }) => theme.page.article.content.textColor};

  p {
    font-size: 16px;
    line-height: 1.8;
    margin-bottom: 16px;
  }

  h2 {
    font-size: 24px;
    font-weight: 600;
    margin: 24px 0 16px;
  }
`;

export const Separator = styled.hr`
  border: none;
  height: 1px;
  background: ${({ theme }) => theme.page.article.separator.backgroundColor};
  margin: 32px 0;
`;

export const CommentsContainer = styled.div`
  padding: 0.5rem;
  max-width: 42rem;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 1rem;
`;
