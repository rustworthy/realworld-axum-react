import { FC, useState } from "react";
import { Button } from "./Button";
import { MainLayout } from "./MainLayout";

const App: FC = () => {
  const [counter, setCounter] = useState<number>(0);

  return (
    <MainLayout>
      <h1> Value: {counter} </h1>

      <Button onClick={() => setCounter((prev) => prev + 1)} dataTestId="increase-button">
        Increase
      </Button>
      <Button onClick={() => setCounter((prev) => prev - 1)} dataTestId="decrease-button">
        Decrease
      </Button>
    </MainLayout>
  );
};

export default App;
