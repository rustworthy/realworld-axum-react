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
`;

export const TabContainer = styled.div`
  display: flex;
  width: 100%;
  border-bottom: 1px solid ${({ theme }) => theme.shared.tabs.container.borderColor};
`;
export const TabList = styled.ul``;

export const TabItem = styled.li`
  float: left;
`;
export const TabLink = styled(Link)<{ $isActive?: boolean }>`
  text-decoration: none;
  display: block;
  padding: 0.5em 1em;
  cursor: ${({ $isActive }) => ($isActive ? "default" : "pointer")};
  border-bottom: 2px solid
    ${({ $isActive, theme }) => ($isActive ? theme.shared.tabs.link.active.borderColor : theme.shared.tabs.link.borderColor)};
  opacity: ${({ $isActive }) => ($isActive ? 1 : 0.5)};

  &:hover {
    opacity: ${({ $isActive }) => ($isActive ? "1" : 0.8)};
    text-decoration: none;
  }
`;

export const TagsContainer = styled.div`
  width: 100%;
  flex: 1;
  padding: 0.5rem 1rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
`;
