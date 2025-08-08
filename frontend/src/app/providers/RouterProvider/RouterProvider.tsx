import { BrowserRouter, Navigate, Outlet, Route, Routes, useLocation } from "react-router";

import { ConfirmEmailPage, ForbiddenPage, HomePage, NotFoundPage, SignInPage, SignUpPage } from "@/pages";
import { ROUTES } from "@/shared/constants/routes.constants";
import { MainLayout } from "@/shared/ui/layouts";

const isAuth = true;

const ProtectedRoutes = () => {
  const location = useLocation();

  return isAuth ? <Outlet /> : <Navigate to={ROUTES.FORBIDDEN} state={{ from: location.pathname }} replace />;
};

export const RouterProvider = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<MainLayout />}>
          <Route path={ROUTES.HOME} element={<HomePage />} />
          <Route path={ROUTES.SIGNIN} element={<SignInPage />} />
          <Route path={ROUTES.SIGNUP} element={<SignUpPage />} />
          <Route path={ROUTES.FORBIDDEN} element={<ForbiddenPage />} />
          <Route path={ROUTES.CONFIRM_EMAIL} element={<ConfirmEmailPage />} />

          <Route element={<ProtectedRoutes />}>{/* <Route path="/protectedroute" element={<ProtectedElement />} /> */}</Route>

          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};
