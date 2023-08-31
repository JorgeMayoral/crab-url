import { Short } from './pages/Short';
import { Check } from './pages/Check';
import { Tab, Tabs } from '@nextui-org/react';
import { Footer } from './components/Footer';

enum Section {
	Short = 'short',
	Long = 'long',
}

function App() {
	return (
		<div className="flex flex-col w-full min-h-screen items-center pt-24 px-10 gap-10">
			<header className="text-center">
				<h1 className="text-5xl font-black">🦀 Crab Url 🦀</h1>
				<p className="mt-2 text-gray-200">Temporal (3h.) Url Shortener</p>
			</header>
			<Tabs>
				<Tab
					className="w-full md:max-w-md"
					key={Section.Short}
					title="Short Url"
				>
					<Short />
				</Tab>
				<Tab
					className="w-full md:max-w-md"
					key={Section.Long}
					title="Check Url"
				>
					<Check />
				</Tab>
			</Tabs>
			<Footer />
		</div>
	);
}

export default App;
