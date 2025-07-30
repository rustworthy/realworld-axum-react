import { Suspense } from "react";
import { Outlet, useLocation, useNavigate } from "react-router";

import * as S from "./MainLayout.styles";

export const MainLayout = () => {
  const navigate = useNavigate();
  const { pathname } = useLocation();

  return (
    <S.Wrapper>
      <S.Header>
        <S.HeaderContainer>
          <S.HeaderLink href="#">conduit</S.HeaderLink>

          <nav>
            <S.HeaderNavList>
              <S.HeaderNavItem $isActive={pathname === "/"} onClick={() => navigate("/")}>
                Home
              </S.HeaderNavItem>
              <S.HeaderNavItem $isActive={pathname === "/signin"} onClick={() => navigate("/signin")}>
                Sign in
              </S.HeaderNavItem>
              <S.HeaderNavItem $isActive={pathname === "/signup"} onClick={() => navigate("/signup")}>
                Sign up
              </S.HeaderNavItem>
            </S.HeaderNavList>
          </nav>
        </S.HeaderContainer>
      </S.Header>

      <S.ChildrenContainer>
        <Suspense fallback={<div>Suspense loader will be here</div>}>
          <Outlet />
        </Suspense>
      </S.ChildrenContainer>

      <S.Footer>
        <S.FooterContainer>
          <S.FooterLink href="https://github.com/rustworthy/realworld-axum-react" target="_blank">
            Fork on GitHub
          </S.FooterLink>
        </S.FooterContainer>
      </S.Footer>
    </S.Wrapper>
  );
};
