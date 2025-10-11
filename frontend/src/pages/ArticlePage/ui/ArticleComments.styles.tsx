import styled from "@emotion/styled";

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

  & > button.SimpleButton {
    height: unset;
    width: fit-content;
    font-size: 1rem;
    padding: 0.5rem 1rem;
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
