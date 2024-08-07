import Input from "./Input/Input";
import { invoke } from "@tauri-apps/api/tauri";
import Grid from "./Grid/Grid";
import { useEffect, useState } from "react";
import { TimeTable } from "./Grid/Grid";

interface GeneratorProps {
  tableNames: string[];
}

const Generator: React.FC<GeneratorProps> = ({ tableNames }) => {
  let [timeTable, setTimeTable] = useState({ classList: [] ,roomSize:0,periodSize:0} as TimeTable);
  let [rooms,SetRooms] = useState([] as string[]);
  let [periods,SetPeriods] = useState([] as string[]);

  useEffect(() => {
    invoke<string[]>("handle_get_rooms").then((res) => {
      SetRooms(res);
    }).catch((err) => {
      SetRooms([err]);
    });

    invoke<string[]>('handle_get_periods').then((res) => {
      SetPeriods(res);
    }).catch((err) => {
      SetPeriods([err]);
    });
  },[timeTable]);

  const sendClassData = () => {
    invoke("handle_set_input");
  };
  const generate = () => {
    invoke("handle_adapt_input");
  };
  const run_once = () => {
    if (timeTable.classList.length != 0){
      invoke("handle_read_cells",{cells:timeTable.classList});
    }
    invoke<TimeTable>("handle_aco_run_once")
      .then((res) => {
        setTimeTable(res);
      })
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <div>
      <button onClick={sendClassData}>convert input</button>
      <button onClick={generate}>set input</button>
      <button onClick={run_once}>next generation</button>
      <Grid timeTable={timeTable} setTimeTable={setTimeTable} rooms={rooms} periods={periods}/>
    </div>
  );
};

export default Generator;
