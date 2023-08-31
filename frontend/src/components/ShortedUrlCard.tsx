import {
	Card,
	CardBody,
	CardFooter,
	CardHeader,
	Divider,
	Link,
	Snippet,
} from '@nextui-org/react';
import { ShortedUrl } from '../interfaces';
import { useEffect, useState } from 'react';

interface Props {
	data: ShortedUrl;
}

export function ShortedUrlCard({ data }: Props) {
	const [timeLeft, setTimeLeft] = useState(data.expire_in);

	useEffect(() => {
		const timer = setInterval(() => {
			setTimeLeft((prev) => prev - 1);
		}, 1000);
		return () => clearInterval(timer);
	}, []);

	const shortedUrl = `https://crab-url.dev/r/${data.id}`;

	return (
		<div className="flex flex-col gap-8 w-full mt-6">
			<Snippet symbol="">{shortedUrl}</Snippet>
			<Card>
				<CardHeader>
					<h3>{data.id}</h3>
				</CardHeader>
				<Divider />
				<CardBody>
					<p>
						This shorted url redirects to:{' '}
						<Link href={data.target} target="_blank">
							{data.target}
						</Link>
					</p>
				</CardBody>
				<Divider />
				<CardFooter>
					<p>Url expires in: {timeLeft} seconds</p>
				</CardFooter>
			</Card>
		</div>
	);
}
