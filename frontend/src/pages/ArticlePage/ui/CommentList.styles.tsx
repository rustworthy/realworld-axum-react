import styled from "@emotion/styled";

export const Comment = styled.div`
  border: 1px solid ${({ theme }) => theme.page.article.comment.borderColor};
  border-radius: 4px;
`;

export const CommentBody = styled.div`
  padding: 16px;
  font-size: 14px;
  line-height: 1.6;
  color: ${({ theme }) => theme.page.article.comment.textColor};
`;

export const CommentFooter = styled.div`
  border-top: 1px solid ${({ theme }) => theme.page.article.comment.borderColor};
  padding: 8px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  font-size: 12px;
  color: ${({ theme }) => theme.page.article.comment.textColor};
  opacity: 0.8;
`;
