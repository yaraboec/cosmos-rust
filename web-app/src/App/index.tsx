import { GlobalStyle } from "../design";
import {
  AccountProvider,
  ErrorProvider,
  SdkProvider,
} from "../logic";
import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { config } from "../config";
import { pathHome, pathLogin, pathOperationResult } from "./paths";
import { ProtectedRoutes } from "./protected-routes";
import { Home } from "./routes/Home";
import { Login } from "./routes/Login";
import { OperationResult } from "./routes/OperationResult";

export function App(): JSX.Element {
  return (
    <ErrorProvider>
      <SdkProvider config={config}>
        <AccountProvider>
          <GlobalStyle />
          <BrowserRouter basename={process.env.PUBLIC_URL}>
            <Routes>
              <Route path="/" element={<Login />} />
              <Route path={pathLogin} element={<Login />} />
                <Route path={pathHome} element={<Home />} />
                <Route
                  path={pathOperationResult}
                  element={<OperationResult />}
                />
            </Routes>
          </BrowserRouter>
        </AccountProvider>
      </SdkProvider>
    </ErrorProvider>
  );
}
