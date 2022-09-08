import React, {useState} from 'react';
import {manager} from "../../declarations/manager/"
import ReactFileReader from "react-file-reader";

const App = () => {

    const [str, setStr] = useState(null)
    const handleFiles = (file) => {

        let str = file.base64.toString()
        console.log(str)
        // manager.set_wasm('data:application/wasm;base64,AGFzbQEAAAAFg4CAgAABABAGkYCAgAACfwBBgIDAAAt/AEGAgMAACwelgICAAAMGbWVtb3J5AgAKX19kYXRhX2VuZAMAC19faGVhcF9iYXNlAwE=')
        manager.create_module(str).then(r => {
            console.log(r)
        })
    }
    return (

        <div>
            {/*<input type="file" name="file" onChange={handleChange}/>*/}
            {/*<div>*/}
            {/*    <button onClick={handleSubmission}>Submit</button>*/}
            {/*</div>*/}


            <ReactFileReader

                fileTypes={[".wasm"]}
                base64
                handleFiles={handleFiles}>
                <button>
                    选择文件
                </button>
            </ReactFileReader>

        </div>

    )
}

export default App;