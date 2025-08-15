import { Suspense } from "react";
import { Outlet, useLocation, useNavigate } from "react-router";

import { useAuth } from "@/shared/auth";
import { useAuthSnapshotRestoration } from "@/shared/auth/hook";
import { ROUTES } from "@/shared/constants/routes.constants";
import { GearIcon, Pencil2Icon } from "@radix-ui/react-icons";
import { useTernaryDarkMode } from "usehooks-ts";

import * as S from "./MainLayout.styles";

export const MainLayout = () => {
  useAuthSnapshotRestoration();
  const navigate = useNavigate();
  const { toggleTernaryDarkMode } = useTernaryDarkMode();
  const { isAuthenticated, user } = useAuth();
  const { pathname } = useLocation();

  return (
    <S.Wrapper>
      <S.Header>
        <S.HeaderContainer>
          <S.HeaderLink href={ROUTES.HOME}>conduit</S.HeaderLink>
          {isAuthenticated ? (
            <nav>
              <S.HeaderNavList>
                <S.HeaderNavItem $isActive={pathname === ROUTES.HOME} onClick={() => navigate(ROUTES.HOME)}>
                  Home
                </S.HeaderNavItem>
                <S.HeaderNavItem $isActive={pathname === ROUTES.EDITOR} onClick={() => navigate(ROUTES.EDITOR)}>
                  <Pencil2Icon />
                  New Article
                </S.HeaderNavItem>
                <S.HeaderNavItem $isActive={pathname === ROUTES.SETTINGS} onClick={() => navigate(ROUTES.SETTINGS)}>
                  <GearIcon />
                  Settings
                </S.HeaderNavItem>
                <S.HeaderNavItem
                  $isActive={pathname === `${ROUTES.PROFILE}/${user!.username}`}
                  onClick={() => navigate(`${ROUTES.PROFILE}/${user!.username}`)}
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
                <S.HeaderNavItem $isActive={pathname === ROUTES.HOME} onClick={() => navigate(ROUTES.HOME)}>
                  Home
                </S.HeaderNavItem>
                <S.HeaderNavItem $isActive={pathname === ROUTES.SIGNIN} onClick={() => navigate(ROUTES.SIGNIN)}>
                  Sign in
                </S.HeaderNavItem>
                <S.HeaderNavItem $isActive={pathname === ROUTES.SIGNUP} onClick={() => navigate(ROUTES.SIGNUP)}>
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
          <S.ThemeSwitch role="button" aria-label="switch color scheme" onClick={toggleTernaryDarkMode} />
          <S.FooterLink href="https://github.com/rustworthy/realworld-axum-react" target="_blank">
            Fork on GitHub
          </S.FooterLink>
        </S.FooterContainer>
      </S.Footer>
    </S.Wrapper>
  );
};
