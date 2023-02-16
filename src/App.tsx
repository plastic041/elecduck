import duckImg from "./assets/duck.png";
import "./App.css";

function App() {
  return (
    <div class="container">
      <img
        src={duckImg}
        class="duck"
        alt="rubber duck"
        data-tauri-drag-region
      />
    </div>
  );
}

export default App;
