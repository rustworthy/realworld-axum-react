import { FC, useState } from "react";
import { Button } from "./Button";

const App: FC = () => {
    const [counter, setCounter] = useState<number>(0);

    return (
        <div style={{width: "300px", textAlign: "center", margin: "50px auto", display: "flex", flexDirection: "column", gap: "16px"}}>
            <h1> Value: {counter} </h1>
            <Button onClick={() => setCounter(prev => prev + 1)} dataTestId="increase-button">Increase</Button>
            <Button onClick={() => setCounter(prev => prev - 1)} dataTestId="decrease-button">Decrease</Button>
        </div>
    );
};

export default App;
