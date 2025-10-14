import { Link } from "react-router";

import styled from "@emotion/styled";

export const TabContainer = styled.div`
  display: flex;
  width: 100%;
  border-bottom: 1px solid ${({ theme }) => theme.shared.tabs.container.borderColor};
`;
export const TabList = styled.ul``;

export const TabItem = styled.li`
  float: left;
`;

export const TabLink = styled(Link, {
  shouldForwardProp: (prop) => prop !== "$isActive",
})<{ $isActive?: boolean }>`
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
