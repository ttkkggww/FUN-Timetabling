import { useDraggable } from "@dnd-kit/core";
import {invoke} from "@tauri-apps/api/tauri";
import { TimeTable } from "../Grid";

interface DraggableProps {
  hex_color: string;
  text: string;
  id: number;
  styles: string;
  classId: number;
  room: number;
  period: number;
  grid_size: number;
  setTimeTable: (
    timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable),
  ) => void;
}

export function Draggable({ hex_color, text, id, styles,room,period,grid_size,setTimeTable }: DraggableProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: id.toString(),
  });
  room = room + 1;
  period = period + 1;
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
        backgroundColor: hex_color,
        gridColumn: `span ${grid_size}`,
        gridArea: `${period}/${room}/${period+grid_size}/${room+1}`,
        zIndex: 3,
      }
    : {
        backgroundColor: hex_color,
        gridColumn: `span ${grid_size}`,
        gridArea: `${period}/${room}/${period+grid_size}/${room+1}`,
        zIndex: 2,
    };

  const handleDobuleClick = () => {
    invoke<TimeTable>("handle_switch_lock", {id:id})
    .then((res) => {
      setTimeTable(res);
    }).catch((err) => {
      console.log(err);
    });
  }
  return (
    <div ref={setNodeRef} 
      {...listeners} 
      {...attributes}
      onDoubleClick={handleDobuleClick}
      style={style} className={styles}>
        {text} 
    </div>
  );
}
