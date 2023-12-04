import { useState } from 'react'
import Logo from './Logo'
import reactLogo from './assets/react.svg'
import './App.css'
import AnimalSearch from "./AnimalSearch";

function App() {
  const [count, setCount] = useState(0)

    return (
        <div className="app">
            <h1>ViZoo!</h1>
            <AnimalSearch />
            {/* Other components or content */}
        </div>
    );
}

export default App
