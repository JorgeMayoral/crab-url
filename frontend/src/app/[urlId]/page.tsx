import { AppNavbar } from '@/components/AppNavbar';

interface UrlIdPageProps {
	params: {
		urlId: string;
	};
}

export default async function UrlIdPage({ params: { urlId } }: UrlIdPageProps) {
	const urlData = await getShortenUrlData(urlId);
	console.log('🚀 ~ file: page.tsx:8 ~ UrlIdPage ~ urlData:', urlData);

	return (
		<div className="h-screen w-screen">
			<AppNavbar />
			<main className="flex flex-col items-center justify-center p-24 w-full h-max">
				<a href={`https://localhost:3000/r/${urlId}`}>
					<h1>https://localhost:3000/r/{urlId}</h1>
				</a>
				<a href={urlData}>
					<h2>Redirects to: {urlData}</h2>
				</a>
			</main>
		</div>
	);
}

async function getShortenUrlData(id: string) {
	const response = await fetch('http://localhost:3000/check', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({ id }),
	});
	const data = await response.text();
	return data;
}
