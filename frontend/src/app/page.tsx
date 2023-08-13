'use client';

import { AppNavbar } from '@/components/AppNavbar';
import { Button, Input } from '@nextui-org/react';
import Link from 'next/link';
import { useState } from 'react';

export default function Home() {
	const [url, setUrl] = useState('');
	const [loading, setLoading] = useState(false);
	const [shortenUrlId, setShortenUrlId] = useState('');

	const handleClick = async () => {
		setLoading(true);
		try {
			const response = await fetch('http://localhost:3000/add', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({ url }),
			});
			const data = await response.text();
			setShortenUrlId(data);
			console.log(data);
		} catch (error) {
			console.error(error);
		}
		setUrl('');
		setLoading(false);
	};

	return (
		<div className="h-screen w-screen">
			<AppNavbar />
			<main className="flex items-center justify-center p-24 w-full h-max">
				<div className="flex flex-col items-center justify-center gap-10 md:max-w-md w-full">
					<Input
						label="Url"
						onChange={(e) => setUrl(e.target.value)}
						value={url}
						type="url"
						size="lg"
					/>
					<Button
						className="bg-gradient-to-tr from-teal-500 to-blue-600 text-white shadow-lg"
						variant="shadow"
						onClick={handleClick}
						isLoading={loading}
						size="lg"
					>
						Shorten
					</Button>
					{shortenUrlId.length > 0 && (
						<div>
							<a href={`https://localhost:3000/r/${shortenUrlId}`}>
								<p className="text-2xl">
									https://localhost:3000/r/{shortenUrlId}
								</p>
							</a>
							<Link href={`/${shortenUrlId}`}>
								<p>See more info</p>
							</Link>
						</div>
					)}
				</div>
			</main>
		</div>
	);
}
