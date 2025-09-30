import { Link } from "react-router";

import styled from "@emotion/styled";

import { TABLET_WIDTH } from "@/shared/constants/styles.constants";
import { LayoutContainer } from "@/shared/ui/Container";

export const PageWrapper = styled.div`
  width: 100%;
  height: 100%;
  overflow-x: hidden;
`;

export const Banner = styled.div`
  height: 170px;
  color: ${({ theme }) => theme.page.home.banner.textColor};
  background-color: ${({ theme }) => theme.page.home.banner.backgroundColor};
`;

export const BannerContainer = styled(LayoutContainer)`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 8px;
`;

export const BannerTitle = styled.h1`
  text-shadow: 0px 1px 3px rgba(0, 0, 0, 0.3);
  font-weight: 700;
  font-size: 56px;
`;

export const BannerDescription = styled.p`
  text-align: center;
  font-weight: 300;
  font-size: 24px;
`;

export const MainContent = styled.div`
  width: 100%;
  padding: 1rem 0.5rem;
  display: flex;
  gap: 1rem;

  @media (max-width: ${TABLET_WIDTH}px) {
    flex-direction: column;
  }
`;

export const FeedContainer = styled.div`
  width: 100%;
  flex: 3;
  display: flex;
  flex-direction: column;
`;

export const PreviewList = styled.div`
  padding-bottom: 1rem;
`;

export const TagsContainer = styled.div`
  width: 100%;
  flex: 1;
  padding: 0.5rem 1rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
`;
