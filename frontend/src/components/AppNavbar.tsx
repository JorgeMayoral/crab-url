'use client';

import { Navbar, NavbarBrand } from '@nextui-org/react';
import Link from 'next/link';

export function AppNavbar() {
	return (
		<Navbar isBordered>
			<NavbarBrand>
				<Link href="/">
					<p className="font-bold">Rust Url Shortener</p>
				</Link>
			</NavbarBrand>
		</Navbar>
	);
}
