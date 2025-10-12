import { Link } from "react-router";

import styled from "@emotion/styled";

import { MOBILE_WIDTH, TABLET_WIDTH } from "@/shared/constants/styles.constants";
import { LayoutContainer } from "@/shared/ui/Container";
import { GitHubLogoIcon } from "@radix-ui/react-icons";

export const Wrapper = styled.div`
  width: 100dvw;
  min-height: 100dvh;
  background-color: ${({ theme }) => theme.mainLayout.backgroundColor};
  color: ${({ theme }) => theme.mainLayout.textColor};
  font-family: "Source Sans Pro", sans-serif;

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
`;

export const Header = styled.header`
  width: 100%;
  height: 4.5rem;
  padding: 1rem 1.5rem;

  @media (max-width: ${TABLET_WIDTH}px) {
    height: 4.75rem;
    padding: 0.75rem 1rem;
  }
`;

export const HeaderContainer = styled(LayoutContainer)`
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 3rem;
`;

export const HeaderLink = styled(Link)`
  font-size: 24px;
  font-family: "Titillium Web", sans-serif;
  color: ${({ theme }) => theme.mainLayout.logo.textColor};
  cursor: pointer;
`;

export const HeaderNavList = styled.ul`
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  flex-wrap: wrap;
  align-items: center;

  @media (max-width: ${TABLET_WIDTH}px) {
    gap: 0.25rem 0.5rem;
    justify-content: space-around;
  }
`;

export const HeaderNavItem = styled.li<{ $isActive?: boolean }>`
  height: 1.8rem;
  width: fit-content;
  display: flex;
  align-items: center;
  justify-content: flex-start;
  gap: 4px;
  cursor: pointer;
  opacity: ${(props) => (props.$isActive ? 1 : 0.5)};
  &:hover {
    opacity: ${(props) => (props.$isActive ? 1 : 0.8)};
  }

  @media (max-width: ${MOBILE_WIDTH}px) {
    width: 7.5rem;
    &.Home {
      display: none;
    }
    &.Compact {
      width: 5rem;
    }
  }
`;

export const ChildrenContainer = styled.main`
  flex-grow: 1;
`;

export const Footer = styled.footer`
  width: 100%;
  height: 66px;
  padding: 15px;
  background: ${({ theme }) => theme.mainLayout.footer.backgroundColor};
  color: ${({ theme }) => theme.mainLayout.footer.textColor};
`;

export const FooterContainer = styled(LayoutContainer)`
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
`;

export const ThemeSwitch = styled(GitHubLogoIcon)`
  width: 1.25rem;
  height: 1.25rem;
`;

export const FooterLink = styled.a`
  font-size: 24px;
  cursor: pointer;
`;
