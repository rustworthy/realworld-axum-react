import styled from "@emotion/styled";

export const CommentForm = styled.form`
  border: 1px solid ${({ theme }) => theme.page.article.comment.borderColor};
  border-radius: 4px;
  background-color: ${({ theme }) => theme.page.article.comment.backgroundColor};
`;

export const CommentFormBody = styled.div`
  padding: 0.75rem 0.75rem 0;
`;

export const CommentFormFooter = styled.div`
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
