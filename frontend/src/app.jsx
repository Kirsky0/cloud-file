import React, {useState} from 'react';
import {manager} from "../declarations/manager"
import {Button} from "@chakra-ui/react";
import ModuleModal from "./modal";
import DataTable from "./DataTable";


const App = () => {


    const handleFile = (e) => {
        const file = e.target.files[0]
        // manager.upload_module(file).then(r => {
        //     console.log(r)
        // })
        const reader = new FileReader()
        reader.readAsArrayBuffer(file)

        reader.onload = function () {
            let uint8Array = new Uint8Array(reader.result);
            manager.upload_module(uint8Array).then(r => {
                console.log(r)
            })
        }
    }

    const [count, setCount] = useState(0)

    const openModal = () => {
        setCount(count + 1)
        // return (<ModuleModal open={true}/>)
    }

    const columns = [
        {
            Header: "module name",
            accessor: "module_name"
        },
        {
            Header: "remark",
            accessor: "remark"
        },
        {
            Header: "upload time",
            accessor: "upload_time"
        }
        // {
        //     Header: "Age",
        //     accessor: "age"
        // },
        // {
        //     Header: "Visits",
        //     accessor: "visits"
        // },
        // {
        //     Header: "Status",
        //     accessor: "status"
        // },
        // {
        //     Header: "Profile Progress",
        //     accessor: "progress"
        // }
    ];


    const data = [
        {
            firstName: 'Hello',
            lastName: 'World',
        },
        {
            firstName: 'react-table',
            lastName: 'rocks',
        },
        {
            firstName: 'whatever',
            lastName: 'you want',
        },]


    return (
        <>

            <DataTable columns={columns} data={data}/>
            <ModuleModal count={count}/>
            <Button onClick={openModal}>open</Button>
        </>

    )
}


export default App;