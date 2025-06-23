import { FC, useState } from "react";

const App: FC = () => {
    const [counter, setCounter] = useState<number>(0);

    return (
        <div style={{width: "300px", textAlign: "center", margin: "50px auto", display: "flex", flexDirection: "column", gap: "16px"}}>
            <h1> Value: {counter} </h1>
            <button onClick={() => setCounter(prev => prev + 1)}>Increase</button>
            <button onClick={() => setCounter(prev => prev - 1)}>Decrease</button>
        </div>
    );
};

export default App;
