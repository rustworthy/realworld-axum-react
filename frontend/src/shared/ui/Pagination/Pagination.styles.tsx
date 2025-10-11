import styled from "@emotion/styled";

export const Pagination = styled.div`
  .SimplePagination {
    font-size: 1rem;
    display: flex;
    gap: 0.5rem;
    padding: 0rem 1rem;

    .Page,
    .PreviousPage,
    .NextPage,
    .Break {
      display: inline-block;
      cursor: pointer;
      padding: 0.5rem 0.75rem;
      text-decoration: none;
      color: ${({ theme }) => theme.shared.pagination.color};
      border: 1px solid ${({ theme }) => theme.shared.pagination.borderColor};

      &.ActivePage {
        background: ${({ theme }) => theme.shared.pagination.active.backgroundColor};
        color: ${({ theme }) => theme.shared.pagination.active.color};
      }
    }

    .NextPage {
      border-bottom-rigkkkht-radius: 0.25rem;
      border-top-right-radius: 0.25rem;
    }

    .PreviousPage {
      border-bottom-left-radius: 0.25rem;
      border-top-left-radius: 0.25rem;
    }

    .PreviousPageListItem,
    .NextPageListItem {
      &.disabled {
        display: none;
      }
    }
  }
`;
