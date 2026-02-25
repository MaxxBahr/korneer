import { useState } from "react";
import Coffee from "./Coffee";
import Container from "react-bootstrap/Container";
import SettingsBar from "./SettingsBar";
import 'bootstrap/dist/css/bootstrap.min.css';

function App() {
  return (
    <Container className="bg-dark">
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
