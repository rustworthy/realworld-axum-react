import styled from "@emotion/styled";

import { LayoutContainer } from "@/shared/styles/globalStyledComponents";

export const Wrapper = styled.div`
  width: 100dvw;
  height: 100dvh;
  background-color: ${({ theme }) => theme.mainLayout.c02};
  color: ${({ theme }) => theme.mainLayout.c01};
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
  color: ${({ theme }) => theme.mainLayout.header.c01};
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
  color: ${(props) => (props.$isActive ? "rgba(0, 0, 0, 0.8)" : "rgba(0, 0, 0, 0.3)")};
  &:hover {
    color: ${(props) => (props.$isActive ? "rgba(0, 0, 0, 0.8)" : "rgba(0, 0, 0, 0.5)")};
  }
`;

export const HeaderAvatar = styled.img`
  width: 32px;
  height: 32px;
  object-fit: cover;
  object-position: top center;
  border-radius: 50%;
`;

export const ChildrenContainer = styled.main`
  flex-grow: 1;
`;

export const Footer = styled.footer`
  width: 100%;
  height: 66px;
  padding: 15px;
  background: ${({ theme }) => theme.mainLayout.footer.c02};
`;

export const FooterContainer = styled(LayoutContainer)`
  display: flex;
  justify-content: center;
  align-items: center;
`;

export const FooterLink = styled.a`
  font-size: 24px;
  color: ${({ theme }) => theme.mainLayout.footer.c01};
  cursor: pointer;
`;
