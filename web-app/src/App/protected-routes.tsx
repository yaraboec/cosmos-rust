import React from "react";
import { Navigate, Routes, RoutesProps } from "react-router-dom";
import { useSdk } from "@cosmicdapp/logic";

interface ProtectedRoutesProps extends RoutesProps {
  readonly authPath: string;
}

export function ProtectedRoutes({
  authPath,
  children,
  location,
}: ProtectedRoutesProps): JSX.Element {
  const { initialized } = useSdk();

  return initialized ? (
    <Routes location={location}>{children}</Routes>
  ) : (
    <Navigate
      to={{
        pathname: authPath,
        search: location ? location.search?.toString() : undefined,
      }}
    />
  );
}
