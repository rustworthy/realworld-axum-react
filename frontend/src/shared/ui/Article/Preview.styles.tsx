import { Link } from "react-router";

import styled from "@emotion/styled";

export const PreviewContainer = styled.div`
  width: 100%;
  padding: 1.5rem 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
`;

export const PreviewMeta = styled.div`
  display: flex;
  justify-content: space-between;
`;

export const PreviewBody = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  line-height: 1.25rem;
  padding-bottom: 0.5rem;
`;

export const PreviewTitle = styled.h2``;

export const PreviewDescription = styled.div`
  opacity: 0.5;
`;

export const ReadMoreLink = styled(Link)`
  opacity: 0.5;
`;

export const PreviewFooter = styled.p`
  display: flex;
  justify-content: space-between;
`;
