import * as React from 'react';
import App from './app';
import {createRoot} from 'react-dom/client';
import {ChakraProvider} from '@chakra-ui/react'

const container = document.getElementById('root');
const root = createRoot(container);
const element = <ChakraProvider>
    <App/>
</ChakraProvider>;

root.render(element);

