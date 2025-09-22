import { Toaster } from "sonner";
import { useTernaryDarkMode } from "usehooks-ts";

export const ToastProvider = () => {
  const { ternaryDarkMode } = useTernaryDarkMode();

  return <Toaster closeButton theme={ternaryDarkMode} duration={10_000} position="top-center" />;
};
