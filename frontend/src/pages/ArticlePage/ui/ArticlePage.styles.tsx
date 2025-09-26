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

export const ArticleMeta = styled.div`
  display: flex;
  align-items: center;
  gap: 2rem;
  flex-wrap: wrap;
`;

export const ArticleActions = styled.div`
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
`;

export const ArticleContent = styled.div`
  margin-bottom: 32px;
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

export const CommentSection = styled.div`
  max-width: 800px;
  margin: 0 auto;
`;

export const CommentForm = styled.form`
  border: 1px solid ${({ theme }) => theme.page.article.comment.borderColor};
  border-radius: 4px;
  margin-bottom: 24px;
  background-color: ${({ theme }) => theme.page.article.comment.backgroundColor};
`;

export const CommentFormBody = styled.div`
  padding: 16px;
`;

export const CommentTextarea = styled.textarea`
  width: 100%;
  border: none;
  resize: vertical;
  min-height: 80px;
  font-family: inherit;
  font-size: 14px;
  background-color: transparent;
  color: ${({ theme }) => theme.page.article.comment.textColor};

  &::placeholder {
    color: ${({ theme }) => theme.shared.input.textColor};
    opacity: 0.7;
  }

  &:focus {
    outline: none;
  }
`;

export const CommentFormFooter = styled.div`
  background: ${({ theme }) => theme.page.article.comment.footerBackgroundColor};
  border-top: 1px solid ${({ theme }) => theme.page.article.comment.borderColor};
  padding: 12px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
`;

export const CommentAuthorImage = styled.img`
  width: 30px;
  height: 30px;
  border-radius: 50%;
`;

export const CommentButton = styled.button`
  background: ${({ theme }) => theme.shared.button.backgroundColor};
  color: ${({ theme }) => theme.shared.button.textColor};
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;

  &:hover {
    background: ${({ theme }) => theme.shared.button.backgroundColorActive};
  }
`;

export const Comment = styled.div`
  border: 1px solid ${({ theme }) => theme.page.article.comment.borderColor};
  border-radius: 4px;
  margin-bottom: 16px;
  background-color: ${({ theme }) => theme.page.article.comment.backgroundColor};
`;

export const CommentBody = styled.div`
  padding: 16px;
  font-size: 14px;
  line-height: 1.6;
  color: ${({ theme }) => theme.page.article.comment.textColor};
`;

export const CommentFooter = styled.div`
  background: ${({ theme }) => theme.page.article.comment.footerBackgroundColor};
  border-top: 1px solid ${({ theme }) => theme.page.article.comment.borderColor};
  padding: 8px 16px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: ${({ theme }) => theme.page.article.comment.textColor};
  opacity: 0.8;
`;

export const CommentAuthor = styled.a`
  text-decoration: none;
  font-weight: 500;

  &:hover {
    text-decoration: underline;
  }
`;

export const CommentDate = styled.span`
  margin-left: auto;
`;

export const ModOptions = styled.span`
  opacity: 0.6;
  cursor: pointer;

  &:hover {
    opacity: 1;
  }
`;
