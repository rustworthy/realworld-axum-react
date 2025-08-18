import { BrowserRouter, Navigate, Outlet, Route, Routes, useLocation } from "react-router";

import { ConfirmEmailPage, EditorPage, HomePage, NotFoundPage, SignInPage, SignUpPage } from "@/pages";
import { MainLayout } from "@/app/layouts";
import { useAuth } from "@/features/auth";
import { SettingsPage } from "@/pages/SettingsPage";
import { ROUTES } from "@/shared/constants/routes.constants";

const ProtectedRoutes = () => {
  const { isAuthenticated } = useAuth();
  const location = useLocation();

  return isAuthenticated ? <Outlet /> : <Navigate to={ROUTES.SIGNIN} state={{ from: location.pathname }} replace />;
};

const AuthRoutes = () => {
  const { isAuthenticated } = useAuth();

  return isAuthenticated ? <Navigate to={ROUTES.HOME} replace /> : <Outlet />;
};

export const RouterProvider = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<MainLayout />}>
          <Route path={ROUTES.HOME} element={<HomePage />} />

          <Route element={<AuthRoutes />}>
            <Route path={ROUTES.SIGNIN} element={<SignInPage />} />
            <Route path={ROUTES.SIGNUP} element={<SignUpPage />} />
            <Route path={ROUTES.CONFIRM_EMAIL} element={<ConfirmEmailPage />} />
          </Route>

          <Route element={<ProtectedRoutes />}>
            <Route path={ROUTES.SETTINGS} element={<SettingsPage />} />
            <Route path={ROUTES.EDITOR} element={<EditorPage />} />
          </Route>

          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};
