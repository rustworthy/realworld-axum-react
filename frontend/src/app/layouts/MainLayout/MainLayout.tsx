import { Suspense } from "react";
import { Outlet } from "react-router";

import { useAuthSnapshotRestoration } from "@/features/auth";
import { useIsGlobalLoading } from "@/features/loading";
import { Loader } from "@/shared/ui/Loader";

import { Footer } from "./Footer";
import { Header } from "./Header/Header";
import * as S from "./MainLayout.styles";
import "./mdeditor.css";

export const MainLayout = () => {
  const isGlobalLoading = useIsGlobalLoading();
  const { isRestoring } = useAuthSnapshotRestoration();
  if (isRestoring) return null;

  return (
    <>
      {isGlobalLoading && <Loader />}

      <S.Wrapper>
        <Header />

        <S.ChildrenContainer>
          <Suspense fallback={<div>Suspense loader will be here</div>}>
            <Outlet />
          </Suspense>
        </S.ChildrenContainer>

        <Footer />
      </S.Wrapper>
    </>
  );
};
