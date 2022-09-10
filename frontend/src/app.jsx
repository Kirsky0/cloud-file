import React, {useState} from 'react';
import {manager} from "../declarations/manager"
import ReactFileReader from "react-file-reader";
import Files from "react-butterfiles";

const App = () => {

    const handleFiles = (file) => {
        console.log(file)
        // let str = file.base64.toString()
        // // manager.set_wasm('data:application/wasm;base64,AGFzbQEAAAAFg4CAgAABABAGkYCAgAACfwBBgIDAAAt/AEGAgMAACwelgICAAAMGbWVtb3J5AgAKX19kYXRhX2VuZAMAC19faGVhcF9iYXNlAwE=')
        // manager.upload_module(str).then(r => {
        //     console.log(r)
        // })
    }
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
        //
        // console.log(uint8Array)
    }
    return (

        <div>
            {/*<input type="file" name="file" onChange={handleChange}/>*/}
            {/*<div>*/}
            {/*    <button onClick={handleSubmission}>Submit</button>*/}
            {/*</div>*/}
            <div>
                <input type="file" accept=".wasm" onChange={e =>
                    handleFile(e)}/>
            </div>

            {/*<ReactFileReader*/}

            {/*    fileTypes={[".wasm"]}*/}
            {/*    base64*/}
            {/*    handleFiles={handleFiles}>*/}
            {/*    <button>*/}
            {/*        选择文件*/}
            {/*    </button>*/}
            {/*</ReactFileReader>*/}

        </div>

    )
}

export default App;