import { useEffect } from 'react';
import reactLogo from './assets/react.svg';
import viteLogo from '/vite.svg';
import './App.css';
import init, { reduce_and_compress_gpx } from './lib/rust/gpx-handler/pkg/gpx_compressor_wasm';
import gpxFile from './lib/data/geo/bareback-to-rickles.gpx?raw';

function App() {
	useEffect(() => {
		init().then(() => {
			console.log('Wasm module loaded');
			reduce_and_compress_gpx(gpxFile);
		});
	}, []);
	return (
		<>
			<div>
				<a href='https://vite.dev' target='_blank'>
					<img src={viteLogo} className='logo' alt='Vite logo' />
				</a>
				<a href='https://react.dev' target='_blank'>
					<img src={reactLogo} className='logo react' alt='React logo' />
				</a>
			</div>
			<h1>Vite + React</h1>
			<div className='card'>
				<p>
					Edit <code>src/App.tsx</code> and save to test HMR
				</p>
			</div>
			<p className='read-the-docs'>Click on the Vite and React logos to learn more</p>
		</>
	);
}

export default App;
