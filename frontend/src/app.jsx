import {TextField, Button, FormControl, InputLabel, Input, FormHelperText, FormGroup} from "@material-ui/core";
import React from 'react';
import {manager} from "../declarations/manager"
import {Controller, useForm} from "react-hook-form";


const App = () => {

    const {control, handleSubmit, reset, setValue,} = useForm({
        defaultValues: {
            module_name: "",
            bytecode: new Uint8Array(),
            remark: ""
        }
    });
    const onSubmit = (data) => {

        console.log(new Date().getTime())
        console.log(data)
    };


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

    const uploadFile = (e) => {
        const file = e.target.files[0]
        // manager.upload_module(file).then(r => {
        //     console.log(r)
        // })
        const reader = new FileReader()
        reader.readAsArrayBuffer(file)

        reader.onload = function () {
            let uint8Array = new Uint8Array(reader.result);
            console.log(uint8Array)
            setValue("bytecode", uint8Array)
        }
    }

    return (
        <div>
            {/*<div>*/}
            {/*    <input type="file" accept=".wasm" onChange={e =>*/}
            {/*        handleFile(e)}/>*/}
            {/*</div>*/}


            <form onSubmit={handleSubmit(onSubmit)}>
                <Controller
                    name="module_name"
                    control={control}
                    render={({field}) => (
                        <FormControl>
                            <InputLabel htmlFor="module name">module name</InputLabel>
                            <Input
                                {...field}
                                label="module name"
                                variant="standard"/>
                        </FormControl>

                    )}
                />
                <Controller
                    name="bytecode"
                    control={control}
                    render={({field: {value, onChange}}) => (
                        <FormGroup>
                            <TextField
                                label="wasm file"
                                type="file"
                                onChange={uploadFile}
                                variant="standard"/>
                        </FormGroup>
                    )}
                />
                <Controller
                    name="remark"
                    control={control}
                    render={({field}) => (
                        <FormControl>
                            <InputLabel htmlFor="remark">remark</InputLabel>
                            <Input
                                {...field}
                                label="remark"
                                variant="standard"/>
                        </FormControl>
                    )}
                />
                <FormControl>
                    <Button onClick={handleSubmit(onSubmit)} variant={"outlined"}>Submit</Button>
                    <Button onClick={() => reset()} variant={"outlined"}>Reset</Button>
                </FormControl>
            </form>
        </div>

    )
}


export default App;