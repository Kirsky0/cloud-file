import React, {useEffect, useState} from 'react';
import {
    Button,
    FormControl, FormLabel, Input,
    Modal,
    ModalBody,
    ModalCloseButton,
    ModalContent, ModalFooter,
    ModalHeader,
    ModalOverlay
} from "@chakra-ui/react";
import {manager} from "../declarations/manager";
import {useFormik} from "formik";

const {useDisclosure} = require("@chakra-ui/react");

const Index = ({count}) => {


    const {isOpen, onOpen, onClose} = useDisclosure()
    const initialRef = React.useRef(null)
    const finalRef = React.useRef(null)


    useEffect(() => onOpen, [count]);


    const [data, setData] = useState({
        moduleName: "",
        bytecode: new Uint8Array(),
        remark: ""
    })
    const [wasmByte, setWasmByte] = useState(new Uint8Array())


    const formik = useFormik({
        initialValues: data,
        onSubmit: (values) => {
            console.log(values)
            console.log(wasmByte)
            console.log(new Date().getTime())

            // manager.upload_module(file).then(r => {
            //     console.log(r)
            // })
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


    return (
        <>
            {/*<Button onClick={onOpen}>Open Modal</Button>*/}
            {/*<Button ml={4} ref={finalRef}>*/}
            {/*    I'll receive focus on close*/}
            {/*</Button>*/}

            <Modal
                initialFocusRef={initialRef}
                finalFocusRef={finalRef}
                isOpen={isOpen}
                onClose={onClose}
            >
                <ModalOverlay/>
                <form onSubmit={formik.handleSubmit}>
                    <ModalContent>
                        <ModalHeader>Create your account</ModalHeader>
                        <ModalCloseButton/>

                        <ModalBody pb={6}>

                            <FormControl mt={4} isRequired>
                                <FormLabel>module name</FormLabel>
                                <Input
                                    id='moduleName'
                                    name='moduleName'
                                    onChange={formik.handleChange}
                                    value={formik.values.moduleName}
                                />
                            </FormControl>


                            <FormControl mt={4} isRequired>
                                <FormLabel>wasm file</FormLabel>
                                <Input
                                    type={"file"}
                                    id='bytecode'
                                    name='bytecode'
                                    onChange={(e) => uploadChange(e)}
                                    value={formik.values.bytecode}
                                />
                            </FormControl>


                            <FormControl mt={4} isRequired>
                                <FormLabel>remark</FormLabel>
                                <Input
                                    id='remark'
                                    name='remark'
                                    onChange={formik.handleChange}
                                    value={formik.values.remark}
                                />
                            </FormControl>

                            {/*<FormControl>*/}
                            {/*    <FormLabel>First name</FormLabel>*/}
                            {/*    <Input ref={initialRef} placeholder='First name'/>*/}
                            {/*</FormControl>*/}

                            {/*<FormControl mt={4}>*/}
                            {/*    <FormLabel>Last name</FormLabel>*/}
                            {/*    <Input placeholder='Last name'/>*/}
                            {/*</FormControl>*/}
                        </ModalBody>

                        <ModalFooter>

                            {/*<Button type='submit'>Submit</Button>*/}


                            <Button type='submit' colorScheme='blue' mr={3}>
                                Submit
                            </Button>
                            <Button onClick={onClose}>Cancel</Button>
                        </ModalFooter>

                    </ModalContent>
                </form>
            </Modal>
        </>
    )
}
export default Index;