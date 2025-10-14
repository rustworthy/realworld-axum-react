import { Link } from "react-router";

import styled from "@emotion/styled";

import { LayoutContainer } from "@/shared/ui/Container";

export const Footer = styled.footer`
  width: 100%;
  height: 66px;
  padding: 15px;
  background: ${({ theme }) => theme.mainLayout.footer.backgroundColor};
  color: ${({ theme }) => theme.mainLayout.footer.textColor};
`;

export const FooterContainer = styled(LayoutContainer)`
  padding: 0 20px;
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 0.5rem;
`;

export const LogoLink = styled(Link)`
  padding-right: 5px;
  font-size: 16px;
  font-family: "Titillium Web", sans-serif;
  color: ${({ theme }) => theme.mainLayout.logo.textColor};
  cursor: pointer;
`;

export const FooterContent = styled.span`
  font-size: 13px;

  a {
    color: ${({ theme }) => theme.mainLayout.logo.textColor};
    cursor: pointer;
  }
`;
