import { useState } from "react";
import Coffee from "./Coffee";
import Container from "react-bootstrap/Container";
import SettingsBar from "./SettingsBar";

function App() {
  return (
    <Container>
      <SettingsBar></SettingsBar>
      <Coffee></Coffee>
      <Coffee></Coffee>
      <Coffee></Coffee>
      <Coffee></Coffee>
      <Coffee></Coffee>
    </Container>
  );
}

export default App;
