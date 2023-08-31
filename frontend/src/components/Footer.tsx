import { Link } from '@nextui-org/react';

export function Footer() {
	return (
		<footer className="mt-10">
			<p>
				Built by{' '}
				<Link href="https://yorch.dev" target="_blank">
					Yorch
				</Link>
			</p>
		</footer>
	);
}
