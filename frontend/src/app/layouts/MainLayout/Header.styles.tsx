import { Link } from "react-router";

import styled from "@emotion/styled";

import { TABLET_WIDTH } from "@/shared/constants/styles.constants";
import { LayoutContainer } from "@/shared/ui/Container";

export const HeaderContainer = styled(LayoutContainer)`
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 3rem;
  padding: 0.75rem 1.25rem;
`;

export const HeaderLink = styled(Link)`
  font-size: 24px;
  font-family: "Titillium Web", sans-serif;
  color: ${({ theme }) => theme.mainLayout.logo.textColor};
  cursor: pointer;
`;

export const DropDownMenuWrapper = styled.div`
  display: none;
  @media (max-width: ${TABLET_WIDTH}px) {
    display: block;
  }

  & button.IconButton {
	all: unset;
  cursor: pointer;
	border-radius: 100%;
	height: 2rem;
	width: 2rem;
	display: inline-flex;
	align-items: center;
	justify-content: center;
  color: ${({ theme }) => theme.mainLayout.textColor};
	border: solid 1px ${({ theme }) => theme.mainLayout.textColor};
	background-color: transparent;
	user-select: none;
`;

export const NavWrapper = styled.nav`
  @media (max-width: ${TABLET_WIDTH}px) {
    display: none;
  }
`;

export const HeaderNavList = styled.ul`
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  flex-wrap: wrap;
  align-items: center;
`;

export const HeaderNavItem = styled.div<{ $isActive?: boolean }>`
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
`;
