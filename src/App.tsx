// App.js

import { useState } from "react";
import TableEditor from "./TableEditor/TableEditor"; // CsvTable コンポーネントのインポート
import styles from "./App.module.css";
import "./react-tab.css";
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";
import { Column } from "react-table";
import Generator from "./Generator/Generator";
import { readDir } from "@tauri-apps/api/fs";
type TableData = {
  columns: Column<object>[];
  data: any[];
};

function App() {
  const tableNames = ["classes", "teachers", "rooms", "studentGroups"];
  return (
    <div className={styles.app}>
      <Tabs>
        <TabList>
          <Tab>時間割生成</Tab>
          {tableNames.map((name,index) => (
            <Tab key={index}>{name}</Tab>
          ))}
        </TabList>

        <TabPanel>
          <Generator tableNames={tableNames}></Generator>
        </TabPanel>
        {tableNames.map((name, i) => (
          <TabPanel key={name}>
            {/* 各タブに CsvTable コンポーネントを配置 */}
            <TableEditor key={i} tableName={name} />
          </TabPanel>
        ))}
      </Tabs>
    </div>
  );
}

export default App;
