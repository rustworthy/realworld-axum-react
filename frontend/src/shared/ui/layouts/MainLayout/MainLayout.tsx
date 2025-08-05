import { Suspense } from "react";
import { Outlet, useLocation, useNavigate } from "react-router";

import { useAuth } from "@/shared/auth";
import { GearIcon, Pencil2Icon } from "@radix-ui/react-icons";

import * as S from "./MainLayout.styles";

export const MainLayout = () => {
  const navigate = useNavigate();
  const { isAuthenticated, user } = useAuth();
  const { pathname } = useLocation();

  return (
    <S.Wrapper>
      <S.Header>
        <S.HeaderContainer>
          <S.HeaderLink href="#">conduit</S.HeaderLink>
          {isAuthenticated ? (
            <nav>
              <S.HeaderNavList>
                <S.HeaderNavItem $isActive={pathname === "/"} onClick={() => navigate("/")}>
                  Home
                </S.HeaderNavItem>
                <S.HeaderNavItem $isActive={pathname === "/editor"} onClick={() => navigate("/editor")}>
                  <Pencil2Icon />
                  New Article
                </S.HeaderNavItem>
                <S.HeaderNavItem $isActive={pathname === "/settings"} onClick={() => navigate("/settings")}>
                  <GearIcon />
                  Settings
                </S.HeaderNavItem>
                <S.HeaderNavItem
                  $isActive={pathname === `/profile/${user!.username}`}
                  onClick={() => navigate(`/profile/${user!.username}`)}
                >
                  <S.HeaderAvatar
                    src={user!.image ?? "https://avatars.githubusercontent.com/u/4324516?v=4"}
                    alt="User's avatar"
                  />
                  {user!.username}
                </S.HeaderNavItem>
              </S.HeaderNavList>
            </nav>
          ) : (
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
          )}
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
