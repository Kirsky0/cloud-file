import React, {useState} from 'react';
import {manager} from "../declarations/manager"
import {
    Button,
    FormControl,
    FormLabel,
    Input,
    VStack
} from "@chakra-ui/react";
import {useFormik} from "formik";
import ModuleModal from "./modal";


const App = () => {

    const [data, setData] = useState({
        moduleName: "",
        bytecode: new Uint8Array(),
        remark: ""
    })
    const [wasmByte, setWasmByte] = useState(new Uint8Array())

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


    const formik = useFormik({
        initialValues: data,
        onSubmit: (values) => {
            console.log(values)
            console.log(wasmByte)
            console.log(new Date().getTime())
        },
    })

    const uploadChange = (e) => {
        const file = e.target.files[0]
        if (file) {
            console.log(file)
            const reader = new FileReader()
            reader.readAsArrayBuffer(file)

            reader.onload = function () {
                let uint8Array = new Uint8Array(reader.result);
                console.log(uint8Array)
                setWasmByte(uint8Array)
            }
        }

        formik.handleChange(e);
    }
    const [count, setCount] = useState(0)

    const openModal = () => {
        setCount(count+1)
        // return (<ModuleModal open={true}/>)
    }
    return (
        <div>
            {/*<div>*/}
            {/*    <input type="file" accept=".wasm" onChange={e =>*/}
            {/*        handleFile(e)}/>*/}
            {/*</div>*/}


            {/*<form onSubmit={handleSubmit(onSubmit)}>*/}
            {/*    <Controller*/}
            {/*        name="module_name"*/}
            {/*        control={control}*/}
            {/*        render={({field}) => (*/}
            {/*            <FormControl>*/}
            {/*                <InputLabel htmlFor="module name">module name</InputLabel>*/}
            {/*                <Input*/}
            {/*                    {...field}*/}
            {/*                    label="module name"*/}
            {/*                    variant="standard"/>*/}
            {/*            </FormControl>*/}

            {/*        )}*/}
            {/*    />*/}
            {/*    <Controller*/}
            {/*        name="bytecode"*/}
            {/*        control={control}*/}
            {/*        render={({field: {value, onChange}}) => (*/}
            {/*            <FormGroup>*/}
            {/*                <TextField*/}
            {/*                    label="wasm file"*/}
            {/*                    type="file"*/}
            {/*                    onChange={uploadFile}*/}
            {/*                    variant="standard"/>*/}
            {/*            </FormGroup>*/}
            {/*        )}*/}
            {/*    />*/}
            {/*    <Controller*/}
            {/*        name="remark"*/}
            {/*        control={control}*/}
            {/*        render={({field}) => (*/}
            {/*            <FormControl>*/}
            {/*                <InputLabel htmlFor="remark">remark</InputLabel>*/}
            {/*                <Input*/}
            {/*                    {...field}*/}
            {/*                    label="remark"*/}
            {/*                    variant="standard"/>*/}
            {/*            </FormControl>*/}
            {/*        )}*/}
            {/*    />*/}
            {/*    <FormControl>*/}
            {/*        <Button onClick={handleSubmit(onSubmit)} variant={"outlined"}>Submit</Button>*/}
            {/*        <Button onClick={() => reset()} variant={"outlined"}>Reset</Button>*/}
            {/*    </FormControl>*/}
            {/*</form>*/}


            <form onSubmit={formik.handleSubmit}>
                <VStack spacing={4} align="flex-start">

                    <FormControl isRequired>
                        <FormLabel>module name</FormLabel>
                        <Input
                            id='moduleName'
                            name='moduleName'
                            onChange={formik.handleChange}
                            value={formik.values.moduleName}
                        />
                    </FormControl>


                    <FormControl isRequired>
                        <FormLabel>wasm file</FormLabel>
                        <Input
                            type={"file"}
                            id='bytecode'
                            name='bytecode'
                            onChange={(e) => uploadChange(e)}
                            value={formik.values.bytecode}
                        />
                    </FormControl>


                    <FormControl isRequired>
                        <FormLabel>remark</FormLabel>
                        <Input
                            id='remark'
                            name='remark'
                            onChange={formik.handleChange}
                            value={formik.values.remark}
                        />
                    </FormControl>

                    <Button type='submit'>Submit</Button>
                </VStack>
            </form>
            <ModuleModal count={count}/>
            <Button onClick={openModal}>open</Button>
        </div>

    )
}


export default App;