import { BrowserRouter, Navigate, Outlet, Route, Routes, useLocation } from "react-router";

import { ConfirmEmailPage, ForbiddenPage, HomePage, NotFoundPage, SignInPage, SignUpPage } from "@/pages";
import { MainLayout } from "@/shared/ui/layouts";

const isAuth = true;

const ProtectedRoutes = () => {
  const location = useLocation();

  return isAuth ? <Outlet /> : <Navigate to="/forbidden" state={{ from: location.pathname }} replace />;
};

export const RouterProvider = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<MainLayout />}>
          <Route path="/" element={<HomePage />} />
          <Route path="/signin" element={<SignInPage />} />
          <Route path="/signup" element={<SignUpPage />} />
          <Route path="/forbidden" element={<ForbiddenPage />} />
          <Route path="/confirm-email" element={<ConfirmEmailPage />} />

          <Route element={<ProtectedRoutes />}>{/* <Route path="/protectedroute" element={<ProtectedElement />} /> */}</Route>

          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};
