import { DESKTOP_CONTAINER_WIDTH, EXTRA_LARGE_WIDTH, LARGE_CONTAINER_WIDTH, LARGE_WIDTH, MOBILE_CONTAINER_WIDTH, TABLET_CONTAINER_WIDTH, TABLET_WIDTH } from "../constants/styles.constants";
import styled from "styled-components";

export const Wrapper = styled.div`
    width: 100dvw;
    height: 100dvh;
    background-color: ${({ theme }) => theme.mainLayout.c02};
    color: ${({ theme }) => theme.mainLayout.c01};
    font-family: "Source Sans Pro", sans-serif;;

    display: flex;
    flex-direction: column;
    justify-content: flex-start;
`;

export const Container = styled.div`
    margin: 0 auto;
    width: 100%;
    max-width: ${DESKTOP_CONTAINER_WIDTH}px;

    @media (max-width: ${EXTRA_LARGE_WIDTH}px) {
        max-width: ${LARGE_CONTAINER_WIDTH}px;
    }
    @media (max-width: ${LARGE_WIDTH}px) {
        max-width: ${TABLET_CONTAINER_WIDTH}px;
    }
    @media (max-width: ${TABLET_WIDTH}px) {
        max-width: ${MOBILE_CONTAINER_WIDTH}px;
    }
`;

export const Header = styled.header`
    width: 100%;
    height: 56px;
    padding: 8px 16px;
    background-color: ${({ theme }) => theme.mainLayout.header.c02};
`;

export const HeaderContainer = styled(Container)`
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

    li {
        cursor: pointer;
    }
`;

export const ChildrenContainer = styled.main`
    flex-grow: 1;
`;

export const Footer = styled.footer`
    width: 100%;
    height: 66px;
    padding: 15px;
    background: ${({ theme }) => theme.mainLayout.footer.c02};
`

export const FooterContainer = styled(Container)`
    display: flex;
    justify-content: center;
    align-items: center;
`

export const FooterLink = styled.a`
    font-size: 24px;
    color: ${({ theme }) => theme.mainLayout.footer.c01};
    cursor: pointer;
`
