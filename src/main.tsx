import React from "react";
import { createRoot } from "react-dom/client";
import { App } from "@/App";
import { SettingsProvider } from "@/contexts/SettingsContext";
import "@/styles/reset.css";
import "@/styles/variables.css";
import "@/styles/globals.css";

const container = document.getElementById("root");
if (!container) throw new Error("Elemento #root não encontrado.");

createRoot(container).render(
  <React.StrictMode>
    <SettingsProvider>
      <App />
    </SettingsProvider>
  </React.StrictMode>,
);
