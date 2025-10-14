import { Suspense } from "react";
import { Outlet } from "react-router";

import { useAuthSnapshotRestoration } from "@/features/auth";
import { useIsGlobalLoading } from "@/shared/store/loading";
import { Loader } from "@/shared/ui/Loader";
import { useTernaryDarkMode } from "usehooks-ts";

import { Header } from "./Header/Header";
import * as S from "./MainLayout.styles";
import "./mdeditor.css";

export const MainLayout = () => {
  const { toggleTernaryDarkMode } = useTernaryDarkMode();
  const isGlobalLoading = useIsGlobalLoading();
  const { isRestoring } = useAuthSnapshotRestoration();
  if (isRestoring) return null;

  return (
    <>
      {isGlobalLoading ? <Loader /> : null}

      <S.Wrapper>
        <Header />

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
