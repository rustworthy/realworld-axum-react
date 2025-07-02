import { BrowserRouter, Routes, Route, Outlet, Navigate, useLocation } from "react-router-dom";
import { MainLayout } from "../MainLayout";
import { HomePage, ForbiddenPage, NotFoundPage, SignInPage, SignUpPage } from "../pages";

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
          <Route path="/singin" element={<SignInPage />} />
          <Route path="/singup" element={<SignUpPage />} />
          <Route path="/forbidden" element={<ForbiddenPage />} />

          <Route element={<ProtectedRoutes />}>
            {/* <Route path="/protectedroute" element={<ProtectedElement />} /> */}
          </Route>

          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};
