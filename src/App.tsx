import cupidKirbyImg from "./assets/cupid_kirby.png";
import "./App.css";

function App() {
  return (
    <div class="container">
      <img
        src={cupidKirbyImg}
        class="kirby"
        alt="rubber duck"
        data-tauri-drag-region
      />
    </div>
  );
}

export default App;
