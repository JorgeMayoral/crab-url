import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';
import { NextUIProvider } from '@nextui-org/react';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<NextUIProvider>
			<main className="dark text-foreground bg-background">
				<App />
			</main>
		</NextUIProvider>
	</React.StrictMode>,
);
