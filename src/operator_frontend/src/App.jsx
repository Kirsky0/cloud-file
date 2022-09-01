import React, {useState} from 'react';
import {operator} from "../../declarations/operator"
import ReactFileReader from "react-file-reader";

const App = () => {

    const [str, setStr] = useState(null)
    const handleFiles = (file) => {

        let str = file.base64.toString()
        console.log(str)
        // operator.set_wasm('data:application/wasm;base64,AGFzbQEAAAAFg4CAgAABABAGkYCAgAACfwBBgIDAAAt/AEGAgMAACwelgICAAAMGbWVtb3J5AgAKX19kYXRhX2VuZAMAC19faGVhcF9iYXNlAwE=')
        operator.set_wasm(str)
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