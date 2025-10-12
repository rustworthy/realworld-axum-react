import { Suspense } from "react";
import { Outlet, useLocation, useNavigate } from "react-router";

import { useAuth, useAuthSnapshotRestoration } from "@/features/auth";
import { ROUTES } from "@/shared/constants/routes.constants";
import { truncateText } from "@/shared/lib/utils";
import { useIsGlobalLoading } from "@/shared/store/loading";
import { Avatar } from "@/shared/ui/Avatar";
import { Loader } from "@/shared/ui/Loader";
import { GearIcon, HomeIcon, Pencil2Icon } from "@radix-ui/react-icons";
import { useTernaryDarkMode } from "usehooks-ts";

import * as S from "./MainLayout.styles";
import "./mdeditor.css";

export const MainLayout = () => {
  const { isRestoring } = useAuthSnapshotRestoration();

  const navigate = useNavigate();
  const { pathname } = useLocation();

  const { toggleTernaryDarkMode } = useTernaryDarkMode();
  const { isAuthenticated, user } = useAuth();
  const isGlobalLoading = useIsGlobalLoading();

  if (isRestoring) return null;

  return (
    <>
      {isGlobalLoading && <Loader />}

      <S.Wrapper>
        <S.Header>
          <S.HeaderContainer>
            <S.HeaderLink to={ROUTES.HOME}>conduit</S.HeaderLink>

            {isAuthenticated ? (
              <nav>
                <S.HeaderNavList>
                  <S.HeaderNavItem className="Compact" $isActive={pathname === ROUTES.HOME} onClick={() => navigate(ROUTES.HOME)}>
                    <HomeIcon />
                    Home
                  </S.HeaderNavItem>
                  <S.HeaderNavItem $isActive={pathname === ROUTES.EDITOR} onClick={() => navigate(ROUTES.EDITOR)}>
                    <Pencil2Icon />
                    New Article
                  </S.HeaderNavItem>
                  <S.HeaderNavItem
                    className="Compact"
                    $isActive={pathname === ROUTES.SETTINGS}
                    onClick={() => navigate(ROUTES.SETTINGS)}
                  >
                    <GearIcon />
                    Settings
                  </S.HeaderNavItem>
                  <S.HeaderNavItem
                    $isActive={pathname === `${ROUTES.PROFILE}/${user!.username}`}
                    onClick={() => navigate(`${ROUTES.PROFILE}/${user!.username}`)}
                  >
                    <Avatar imageUrl={user!.image} username={user!.username} />
                    {truncateText(user!.username, 12)}
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
    </>
  );
};
