import { FC } from "react";
import ReactPaginate, { ReactPaginateProps } from "react-paginate";

import * as S from "./Pagination.styles";

export type PaginationProps = {
  onPageChange: ReactPaginateProps["onPageChange"];
  pageCount: ReactPaginateProps["pageCount"];
};

export const Pagination: FC<PaginationProps> = ({ onPageChange, pageCount }) => {
  return (
    <S.Pagination>
      <ReactPaginate
        className="SimplePagination"
        pageLinkClassName="Page"
        activeLinkClassName="ActivePage"
        previousLinkClassName="PreviousPage"
        previousClassName="PreviousPageListItem"
        nextLinkClassName="NextPage"
        nextClassName="NextPageListItem"
        breakLabel="..."
        nextLabel=">"
        pageRangeDisplayed={2}
        marginPagesDisplayed={2}
        onPageChange={onPageChange}
        pageCount={pageCount}
        previousLabel="<"
        renderOnZeroPageCount={null}
      />
    </S.Pagination>
  );
};
