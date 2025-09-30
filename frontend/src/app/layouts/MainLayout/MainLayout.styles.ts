import styled from "@emotion/styled";

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
  height: 56px;
  padding: 8px 16px;
`;

export const HeaderContainer = styled(LayoutContainer)`
  display: flex;
  justify-content: space-between;
  align-items: center;
`;

export const HeaderLink = styled.a`
  font-size: 24px;
  font-family: "Titillium Web", sans-serif;
  color: ${({ theme }) => theme.mainLayout.logo.textColor};
  cursor: pointer;
`;

export const HeaderNavList = styled.ul`
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 16px;
`;

export const HeaderNavItem = styled.li<{ $isActive?: boolean }>`
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  opacity: ${(props) => (props.$isActive ? 1 : 0.5)};
  &:hover {
    opacity: ${(props) => (props.$isActive ? 1 : 0.8)};
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
